#!/usr/bin/env node

/**
 * 多语言键提取脚本
 * 
 * 这个脚本会扫描src目录下的所有文件，提取所有的多语言键，
 * 并将它们整理到语言包文件中。
 * 
 * 支持的提取模式：
 * - $t('key')
 * - $t('key', { name: 'value' })
 * - v-i18n="'key'"
 * - v-i18n="{ key: 'key' }"
 */

const fs = require('fs');
const path = require('path');
const { glob } = require('glob');

// 获取项目根目录
const rootDir = path.resolve(__dirname, '..');

// 加载配置文件
let config;
try {
  const configPath = path.resolve(rootDir, 'i18n.config.js');
  if (fs.existsSync(configPath)) {
    config = require(configPath);
    console.log('已加载配置文件:', configPath);
  } else {
    console.log('未找到配置文件，使用默认配置');
    config = {};
  }
} catch (error) {
  console.error('加载配置文件时出错:', error);
  console.log('使用默认配置');
  config = {};
}

// 配置项
const CONFIG = {
  // 源代码目录
  srcDir: path.resolve(rootDir, config.srcDir || './src'),
  // 语言包目录
  localesDir: path.resolve(rootDir, config.outputDir || './src/i18n/locales'),
  // 输出格式: 'ts' 或 'json'
  outputFormat: config.outputFormat === 'json' ? 'json' : 'ts',
  // 支持的语言
  locales: config.locales || ['zh-CN', 'en-US'],
  // 文件匹配模式
  filePatterns: config.filePatterns || ['**/*.vue', '**/*.ts', '**/*.js', '**/*.jsx', '**/*.tsx'],
  // 忽略的目录
  ignoreDirs: config.ignoreDirs || ['node_modules', 'dist', 'public'],
  // 忽略的文件
  ignoreFiles: config.ignoreFiles || [],
  // 是否保留已有的翻译
  preserveExistingTranslations: config.preserveExistingTranslations !== undefined ? config.preserveExistingTranslations : true,
  // 是否按命名空间组织键
  organizeByNamespace: config.organizeByNamespace !== undefined ? config.organizeByNamespace : true,
  // 默认值（当找不到翻译时）
  defaultValue: config.defaultValue !== undefined ? config.defaultValue : '',
  // 提取模式配置
  patterns: config.patterns || {
    getLang: true,
    tFunction: true,
    vI18nString: true,
    vI18nObject: true,
    customPatterns: []
  }
};

// 打印配置信息
console.log('配置信息:');
console.log('- 源代码目录:', CONFIG.srcDir);
console.log('- 语言包目录:', CONFIG.localesDir);
console.log('- 支持的语言:', CONFIG.locales.join(', '));
console.log('- 文件匹配模式:', CONFIG.filePatterns.join(', '));
console.log('- 忽略的目录:', CONFIG.ignoreDirs.join(', '));
console.log('- 保留已有翻译:', CONFIG.preserveExistingTranslations);
console.log('- 按命名空间组织:', CONFIG.organizeByNamespace);
console.log('- 输出格式:', CONFIG.outputFormat);

// 构建正则表达式
const buildPatterns = () => {
  const patterns = [];

  // 添加内置模式
  if (CONFIG.patterns.tFunction) {
    patterns.push({
      name: '$t函数',
      // 修改正则表达式，匹配更多情况，包括模板中的$t调用
      regex: /(?:\$t)\(\s*['"]([^'"]+)['"]/g
    });

    // 添加GetLang函数模式
    patterns.push({
      name: 'GetLang函数',
      regex: /GetLang\(\s*['"]([^'"]+)['"]/g
    });
    patterns.push({
      name: 't函数',
      regex: /[\s]+t\(\s*['"]([^'"]+)['"]/g
    });
  }


  if (CONFIG.patterns.vI18nString) {
    patterns.push({
      name: 'v-i18n字符串',
      regex: /v-i18n\s*=\s*['"]'([^'"]+)'['"]/g
    });
  }

  if (CONFIG.patterns.vI18nObject) {
    patterns.push({
      name: 'v-i18n对象',
      regex: /v-i18n\s*=\s*['"]\{\s*key\s*:\s*['"]([^'"]+)['"]/g
    });
  }

  // 添加自定义模式
  if (CONFIG.patterns.customPatterns && CONFIG.patterns.customPatterns.length > 0) {
    CONFIG.patterns.customPatterns.forEach((pattern, index) => {
      if (typeof pattern === 'string') {
        try {
          // 将字符串转换为正则表达式
          const flags = pattern.replace(/.*\/([gimy]*)$/, '$1');
          const regex = new RegExp(pattern.replace(new RegExp('^/(.*?)/' + flags + '$'), '$1'), flags);
          patterns.push({
            name: `自定义模式 ${index + 1}`,
            regex
          });
        } catch (error) {
          console.error(`无效的正则表达式: ${pattern}`, error);
        }
      } else if (pattern instanceof RegExp) {
        patterns.push({
          name: `自定义模式 ${index + 1}`,
          regex: pattern
        });
      }
    });
  }

  return patterns;
};

const PATTERNS = buildPatterns();

/**
 * 扫描文件，提取多语言键
 * @param {string} filePath 文件路径
 * @returns {string[]} 提取到的多语言键数组
 */
function extractKeysFromFile(filePath) {
  let content = fs.readFileSync(filePath, 'utf-8');
  const keys = new Set();
  let matchCount = 0;

  // 预处理：移除注释
  content = removeComments(content, filePath);

  // 使用所有模式提取键
  PATTERNS.forEach(pattern => {
    let match;
    let patternMatchCount = 0;
    const regex = pattern.regex;

    // 重置正则表达式的lastIndex
    regex.lastIndex = 0;
    // if(filePath.includes("map.ts")){
    //   console.log(filePath,regex)
    // }

    while ((match = regex.exec(content)) !== null) {
      const key = match[1];
      keys.add(key);
      // if(filePath.includes("map.ts")){
      //   console.log(filePath,key)
      // }
      // if (key == "--") {
      //   console.log(filePath, "---", pattern.regex);
      // }
      patternMatchCount++;
      matchCount++;
      // 添加调试输出
      // console.log(`  - 在文件 ${path.relative(rootDir, filePath)} 中找到键: "${key}" (使用模式: ${pattern.name})`);
      // console.log(`    原始匹配: "${match[0]}"`);
    }

    if (patternMatchCount > 0) {
      console.log(`  - 在 ${path.relative(rootDir, filePath)} 中找到 ${patternMatchCount} 个键 (${pattern.name})`);
    }
  });

  return { keys: Array.from(keys), matchCount };
}

/**
 * 移除文件内容中的注释
 * @param {string} content 文件内容
 * @param {string} filePath 文件路径
 * @returns {string} 移除注释后的内容
 */
function removeComments(content, filePath) {
  const fileExt = path.extname(filePath).toLowerCase();

  // 根据文件类型选择不同的注释处理方式
  if (['.vue', '.js', '.ts', '.jsx', '.tsx'].includes(fileExt)) {
    // 处理 JavaScript/TypeScript/Vue 文件

    // 移除单行注释 (// ...)
    content = content.replace(/\/\/.*$/gm, '');

    // 移除多行注释 (/* ... */)
    content = content.replace(/\/\*[\s\S]*?\*\//g, '');

    if (fileExt === '.vue') {
      // 移除 Vue 文件中的 HTML 注释 (<!-- ... -->)
      content = content.replace(/<!--[\s\S]*?-->/g, '');
    }
  } else if (['.html', '.xml'].includes(fileExt)) {
    // 处理 HTML/XML 文件
    content = content.replace(/<!--[\s\S]*?-->/g, '');
  }

  return content;
}

/**
 * 处理键名中的特殊字符
 * @param {string} key 键名
 * @returns {string} 处理后的键名
 */
function sanitizeKey(key) {
  // 将连字符替换为下划线
  return key.replace(/[\s-]/g, '_');
}

/**
 * 检查字符串是否是有效的JavaScript标识符
 * @param {string} str 要检查的字符串
 * @returns {boolean} 是否是有效的JavaScript标识符
 */
function isValidJSIdentifier(str) {
  // JavaScript标识符必须以字母、下划线或$开头，后面可以跟字母、数字、下划线或$
  return /^[a-zA-Z0-9_$]*$/.test(str);
}

/**
 * 将键按命名空间组织成嵌套对象
 * @param {string[]} keys 多语言键数组
 * @returns {object} 按命名空间组织的对象
 */
function organizeKeysByNamespace(keys) {
  const result = {};

  keys.forEach(key => {
    const parts = key.split('.');
    let current = result;

    for (let i = 0; i < parts.length - 1; i++) {
      const part = parts[i];
      // 只有在键名不是有效的JavaScript标识符时才处理
      const sanitizedPart = !isValidJSIdentifier(part) ? sanitizeKey(part) : part;
      if (!isValidJSIdentifier(sanitizedPart)) {
        console.log(`${key} 中的 ${part} 替换后${sanitizedPart} 仍然不是有效的JavaScript标识符`);
        continue;
      }
      if (!current[sanitizedPart]) {
        current[sanitizedPart] = {};
      }
      current = current[sanitizedPart];
    }

    const lastPart = parts[parts.length - 1];
    // 只有在键名不是有效的JavaScript标识符时才处理
    const sanitizedLastPart = !isValidJSIdentifier(lastPart) ? sanitizeKey(lastPart) : lastPart;

    if (!current[sanitizedLastPart]) {
      current[sanitizedLastPart] = CONFIG.defaultValue;
    }
  });

  return result;
}

/**
 * 深度合并两个对象
 * @param {object} target 目标对象
 * @param {object} source 源对象
 * @returns {object} 合并后的对象
 */
function deepMerge(target, source) {
  const result = { ...target };

  for (const key in source) {
    if (source.hasOwnProperty(key)) {
      if (
        typeof source[key] === 'object' &&
        source[key] !== null &&
        typeof target[key] === 'object' &&
        target[key] !== null
      ) {
        result[key] = deepMerge(target[key], source[key]);
      } else if (CONFIG.preserveExistingTranslations && target[key] !== undefined) {
        // 保留已有的翻译
        result[key] = target[key];
      } else {
        result[key] = source[key];
      }
    }
  }

  return result;
}

/**
 * 将对象转换为格式化的JavaScript代码
 * @param {object} obj 要转换的对象
 * @param {number} indent 缩进级别
 * @returns {string} 格式化的JavaScript代码
 */
function formatObjectToJS(obj, indent = 0) {
  const spaces = ' '.repeat(indent * 2);
  const lines = ['{'];

  for (const key in obj) {
    if (obj.hasOwnProperty(key)) {
      const value = obj[key];
      // 只有在键名包含特殊字符且不是有效的JavaScript标识符时才添加引号
      if (typeof value === 'object' && value !== null) {
        lines.push(`${spaces}  ${key}: ${formatObjectToJS(value, indent + 1)},`);
      } else if (typeof value === 'string') {
        lines.push(`${spaces}  ${key}: '${value.replace(/'/g, "\\'")}',`);
      } else {
        lines.push(`${spaces}  ${key}: ${value},`);
      }
    }
  }

  lines.push(`${spaces}}`);
  return lines.join('\n');
}

/**
 * 更新语言包文件
 * @param {string} locale 语言代码
 * @param {object} keys 多语言键对象
 */
function updateLocaleFile(locale, keys) {
  const ext = CONFIG.outputFormat === 'json' ? 'json' : 'ts';
  const filePath = path.join(CONFIG.localesDir, `${locale}.${ext}`);
  let existingKeys = {};

  // 如果文件存在，读取已有的键
  if (fs.existsSync(filePath)) {
    try {
      const content = fs.readFileSync(filePath, 'utf-8');
      if (CONFIG.outputFormat === 'json') {
        existingKeys = JSON.parse(content);
      } else {
        const match = content.match(/export\s+default\s+(\{[\s\S]*\})/);
        if (match) {
          // 警告：这里使用eval，仅用于开发工具，不要在生产环境使用
          // 更安全的方法是使用专门的解析器
          existingKeys = eval(`(${match[1]})`);
        }
      }
    } catch (error) {
      console.error(`Error reading locale file ${filePath}:`, error);
    }
  }

  // 合并已有的键和新提取的键
  const mergedKeys = deepMerge(existingKeys, keys);

  // 生成新的文件内容
  const content = CONFIG.outputFormat === 'json'
    ? JSON.stringify(mergedKeys, null, 2)
    : `export default ${formatObjectToJS(mergedKeys, 0)}`;

  // 确保目录存在
  if (!fs.existsSync(CONFIG.localesDir)) {
    fs.mkdirSync(CONFIG.localesDir, { recursive: true });
  }

  // 写入文件
  fs.writeFileSync(filePath, content, 'utf-8');
  console.log(`更新语言包文件: ${path.relative(rootDir, filePath)}`);
}

/**
 * 主函数
 */
async function main() {
  console.log('开始提取多语言键...');

  // 获取所有匹配的文件
  const files = [];
  // 将输出目录（如果在 src 下）加入忽略列表，避免扫描已生成的语言文件
  try {
    const relFromSrc = path.relative(CONFIG.srcDir, CONFIG.localesDir).replace(/\\/g, '/');
    if (relFromSrc && !relFromSrc.startsWith('..')) {
      CONFIG.ignoreDirs.push(relFromSrc);
    }
  } catch {}

  // 修正忽略目录的处理方式
  const ignorePatterns = [
    ...CONFIG.ignoreDirs.map(dir => {
      // 确保dir是字符串
      if (!dir) return '';

      // 如果是相对路径，确保正确处理
      if (typeof dir === 'string' && (dir.startsWith('src/') || dir.startsWith('./src/'))) {
        return `**/${dir.replace(/^\.\//, '')}/**`;
      }
      return `**/${dir}/**`;
    }).filter(Boolean),
    ...CONFIG.ignoreFiles
  ];

  console.log('- 忽略模式:', ignorePatterns.join(', '));

  for (const pattern of CONFIG.filePatterns) {
    const matches = await glob(pattern, {
      cwd: CONFIG.srcDir,
      ignore: ignorePatterns,
      absolute: true,
    });
    files.push(...matches);
  }

  console.log(`找到 ${files.length} 个文件进行扫描.`);

  // 提取所有键
  const allKeys = new Set();
  let totalMatchCount = 0;

  console.log('开始扫描文件:');
  files.forEach(file => {
    const { keys, matchCount } = extractKeysFromFile(file);
    keys.forEach(key => allKeys.add(key));
    totalMatchCount += matchCount;
  });

  console.log(`共提取到 ${allKeys.size} 个唯一的多语言键，总共 ${totalMatchCount} 处引用.`);

  // 按命名空间组织键
  const organizedKeys = organizeKeysByNamespace(Array.from(allKeys));

  // 更新每个语言的文件
  CONFIG.locales.forEach(locale => {
    updateLocaleFile(locale, organizedKeys);
  });

  console.log('多语言键提取完成!');
}

// 执行主函数
main().catch(error => {
  console.error('提取多语言键时出错:', error);
  process.exit(1);
}); 