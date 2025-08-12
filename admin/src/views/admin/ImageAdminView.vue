<template>
  <div class="p-4 md:p-6 space-y-4">
    <!-- Toolbar -->
    <div class="flex flex-wrap items-center gap-2">
      <div class="flex items-center gap-2">
        <button class="border border-gray-300 rounded px-2 py-1 text-sm bg-white hover:bg-gray-50 disabled:opacity-50" @click="goBack" title="Back" :disabled="!canGoBack">←</button>
        <button class="border border-gray-300 rounded px-2 py-1 text-sm bg-white hover:bg-gray-50 disabled:opacity-50" @click="goUp" title="Up" :disabled="!canGoUp">↑</button>
        <button class="border border-gray-300 rounded px-2 py-1 text-sm bg-white hover:bg-gray-50" @click="resetAndLoad" title="Home (/)">⌂</button>
      </div>
      <div class="flex-1 min-w-[220px] max-w-[680px]">
        <div class="flex items-center gap-1 text-sm bg-white border border-gray-200 rounded px-2 py-1">
          <span class="cursor-pointer hover:text-blue-600" @click="navigateTo('/')">This PC</span>
          <template v-for="seg in breadcrumb.segments" :key="seg.full">
            <span class="text-gray-400 px-1">›</span>
            <span class="cursor-pointer hover:text-blue-600" @click="navigateTo(seg.full)">{{ seg.name }}</span>
          </template>
        </div>
      </div>
      <div class="flex items-center gap-2">
        <input v-model="state.query.name" placeholder="Name" class="border border-gray-300 rounded px-2 py-1 text-sm w-40" />
        <input v-model="state.query.object_key" placeholder="Object Key" class="border border-gray-300 rounded px-2 py-1 text-sm w-56" />
        <input v-model="state.query.path" placeholder="Path (e.g. /, /banners)" class="border border-gray-300 rounded px-2 py-1 text-sm w-56" />
        <button class="px-3 py-1.5 rounded text-white bg-blue-600 hover:bg-blue-700 text-sm" @click="loadImages">Search</button>
        <button class="px-3 py-1.5 rounded border border-gray-300 text-sm" @click="refresh">Refresh</button>
        <button class="px-3 py-1.5 rounded text-white bg-green-600 hover:bg-green-700 text-sm" @click="openCreateDialog">New</button>
      </div>
    </div>

    <!-- Content Area -->
    <div class="border rounded bg-white overflow-hidden">
      <div class="p-2 border-b text-xs text-gray-500 bg-gray-50">{{ currentPath || '/' }}</div>
      <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 xl:grid-cols-8 gap-3 p-3">
        <!-- Folders -->
        <div v-for="folder in folders" :key="folder" class="rounded p-2 hover:bg-blue-50 cursor-default group" @dblclick="openFolder(folder)" @click="selectTile({ type: 'folder', name: folder })">
          <div class="w-full aspect-[4/3] bg-yellow-50 rounded relative overflow-hidden flex items-center justify-center">
            <svg viewBox="0 0 24 24" class="w-10 h-10 text-yellow-500"><path fill="currentColor" d="M10 4H4a2 2 0 0 0-2 2v2h20V8a2 2 0 0 0-2-2h-8l-2-2Z"/><path fill="currentColor" d="M22 10H2v8a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2Z"/></svg>
          </div>
          <div class="mt-2 text-xs text-gray-800 truncate" :title="folder">{{ folder }}</div>
        </div>

        <!-- Files -->
        <div v-for="img in files" :key="img.id" class="rounded p-2 hover:bg-blue-50 cursor-default group" @dblclick="edit(img)" @click="selectTile({ type: 'file', id: img.id })">
          <div class="w-full aspect-[4/3] bg-gray-100 rounded relative overflow-hidden flex items-center justify-center">
            <img :src="img.url" :alt="img.name" class="w-full h-full object-cover" />
            <div class="absolute inset-0 flex items-end justify-center gap-2 p-2 opacity-0 transition-opacity bg-gradient-to-t from-black/50 to-transparent group-hover:opacity-100">
              <button class="text-xs px-2 py-0.5 border border-white/70 rounded bg-black/40 text-white" @click.stop="copy(img.url)">Copy URL</button>
              <button class="text-xs px-2 py-0.5 border border-white/70 rounded bg-black/40 text-white" @click.stop="edit(img)">Edit</button>
              <button class="text-xs px-2 py-0.5 border border-white/70 rounded bg-red-600 text-white" @click.stop="del(img.id)">Delete</button>
            </div>
          </div>
          <div class="mt-2 text-xs text-gray-800 truncate" :title="img.name">{{ img.name }}</div>
        </div>
      </div>
      <div v-if="state.total === 0" class="p-8 text-center text-gray-500">No items</div>
    </div>

    <!-- Footer / Pagination -->
    <div class="flex items-center justify-between text-sm">
      <div>Items: {{ state.total }}</div>
      <div class="flex items-center gap-2">
        <button class="px-3 py-1.5 rounded border border-gray-300 text-sm disabled:opacity-50" :disabled="state.page<=1" @click="prevPage">Prev</button>
        <span>Page {{ state.page }}</span>
        <button class="px-3 py-1.5 rounded border border-gray-300 text-sm disabled:opacity-50" :disabled="files.length + folders.length < state.page_size" @click="nextPage">Next</button>
      </div>
    </div>

    <!-- Dialog (Element Plus) -->
    <el-dialog v-model="formVisible" :title="form.id ? 'Edit Image' : 'Create Image'" width="600">
      <el-form label-width="130px">
        <el-form-item label="Name"><el-input v-model="form.name" /></el-form-item>
        <el-form-item label="Object Key"><el-input v-model="form.object_key" /></el-form-item>
        <el-form-item label="URL"><el-input v-model="form.url" /></el-form-item>
        <el-form-item label="Path"><el-input v-model="form.path" /></el-form-item>
        <el-form-item label="Upload">
          <el-upload :before-upload="beforeUpload" :http-request="handleOssUpload" :show-file-list="false" accept="image/*">
            <el-button type="primary">Select Image</el-button>
            <template #tip><div class="text-xs text-gray-500 mt-1">Will upload to OSS then fill URL/Object Key automatically.</div></template>
          </el-upload>
        </el-form-item>
        <el-form-item label="Tags">
          <el-input-tag v-model="tagsList" delimiter="," />
        </el-form-item>
        <el-form-item label="Status">
          <el-select v-model="form.status" style="width: 160px">
            <el-option label="Enabled" :value="1" />
            <el-option label="Disabled" :value="0" />
          </el-select>
        </el-form-item>
        <el-form-item label="Remark"><el-input v-model="form.remark" type="textarea" :rows="3" /></el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="formVisible = false">Cancel</el-button>
        <el-button type="primary" @click="save">Save</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref, computed, onMounted } from 'vue'
import { fetchImages, createImage, updateImage, deleteImage } from '@/apis/images'
import type { ImageModel, AddImageReq, UpdateImageReq, ListImagesParams } from '@/types'
import { ElMessage, ElMessageBox, ElInputTag } from 'element-plus'
import { createOssClient, uploadFile, genObjectKey } from '@/utils/oss'

const state = reactive({
  items: [] as ImageModel[],
  total: 0,
  page: 1,
  page_size: 24,
  query: { path: '/', name: '', object_key: '' } as Partial<ListImagesParams>,
})

const currentPath = computed(() => state.query.path || '/')

// History stack for Back
const navHistory: string[] = []
const canGoBack = computed(() => navHistory.length > 0)
const canGoUp = computed(() => normalize(currentPath.value) !== '/')

const breadcrumb = computed(() => {
  const path = normalize(currentPath.value)
  if (path === '/') return { segments: [] as { name: string, full: string }[] }
  const parts = path.replace(/^\//, '').split('/')
  const segments: { name: string, full: string }[] = []
  let acc = ''
  for (const p of parts) {
    acc += '/' + p
    segments.push({ name: p, full: acc })
  }
  return { segments }
})

async function loadImages() {
  const data = await fetchImages({
    page: state.page,
    page_size: state.page_size,
    path: state.query.path,
    name: state.query.name,
    object_key: state.query.object_key,
  })
  state.items = data.list
  state.total = data.total
}

function resetAndLoad() {
  state.page = 1
  state.query = { path: '/' }
  loadImages()
}

function refresh() { loadImages() }

function prevPage() {
  if (state.page > 1) {
    state.page--
    loadImages()
  }
}
function nextPage() {
  state.page++
  loadImages()
}

const formVisible = ref(false)
const tagsList = ref<string[]>([])
const form = reactive<Partial<ImageModel & AddImageReq & UpdateImageReq>>({
  name: '', object_key: '', url: '', path: '/', tags: [], status: 1, remark: ''
})

function openCreateDialog() {
  Object.assign(form, { id: undefined, name: '', object_key: '', url: '', path: state.query.path || '/', tags: [], status: 1, remark: '' })
  tagsList.value = []
  formVisible.value = true
}
function edit(img: ImageModel) {
  Object.assign(form, img)
  tagsList.value = [...(img.tags || [])]
  formVisible.value = true
}

async function save() {
  const tags = tagsList.value
  if (form.id) {
    await updateImage(form.id as number, {
      name: form.name,
      object_key: form.object_key,
      url: form.url,
      path: form.path,
      tags,
      status: form.status,
      remark: form.remark,
    })
  } else {
    await createImage({
      name: form.name as string,
      object_key: form.object_key as string,
      url: form.url as string,
      path: form.path as string,
      tags,
      status: form.status as number,
      remark: form.remark as string | undefined,
    })
  }
  formVisible.value = false
  loadImages()
}

async function del(id: number) {
  await ElMessageBox.confirm('Delete this image?')
  await deleteImage(id)
  ElMessage.success('Deleted')
  loadImages()
}

// OSS upload
const beforeUpload = (file: File) => {
  const isImage = /^image\//.test(file.type)
  if (!isImage) { ElMessage.error('Only image files'); return false }
  return true
}
const handleOssUpload = async (options: any) => {
  try {
    // 获取 STS（后端已实现 /api/admin/storage/oss/sts）
    // 这里简单用环境变量直连，也可以调用后端接口获取临时凭证（更安全）
    const client = createOssClient({
      region: import.meta.env.VITE_OSS_REGION,
      bucket: import.meta.env.VITE_OSS_BUCKET,
      accessKeyId: import.meta.env.VITE_OSS_ACCESS_KEY_ID,
      accessKeySecret: import.meta.env.VITE_OSS_ACCESS_KEY_SECRET,
      secure: true,
      endpoint: import.meta.env.VITE_OSS_ENDPOINT,
    } as any)
    const key = genObjectKey('uploads/images', options.file as File)
    const url = await uploadFile(client as any, options.file as File, key)
    form.object_key = key
    form.url = url
    ElMessage.success('Uploaded')
    options.onSuccess?.({}, options.file)
  } catch (e) {
    ElMessage.error('Upload failed')
    options.onError?.(e)
  }
}

onMounted(loadImages)

// Explorer helpers
function normalize(path?: string | null) {
  if (!path || path.trim() === '' || path === '/') return '/'
  let p = path.trim()
  if (!p.startsWith('/')) p = '/' + p
  if (p.length > 1 && p.endsWith('/')) p = p.slice(0, -1)
  return p
}

const baseWithSlash = computed(() => {
  const base = normalize(currentPath.value)
  return base === '/' ? '/' : base + '/'
})

const folders = computed(() => {
  const set = new Set<string>()
  const base = normalize(currentPath.value)
  const baseSlash = baseWithSlash.value
  for (const it of state.items) {
    const p = normalize(it.path)
    if (p === base) continue
    if (base === '/') {
      // root: first segment
      const seg = p.replace(/^\//, '').split('/')[0]
      if (seg) set.add(seg)
    } else if (p.startsWith(baseSlash)) {
      const rest = p.slice(baseSlash.length)
      const seg = rest.split('/')[0]
      if (seg) set.add(seg)
    }
  }
  return Array.from(set).sort()
})

const files = computed(() => {
  const base = normalize(currentPath.value)
  return state.items.filter(it => normalize(it.path) === base)
})

function openFolder(name: string) {
  const base = normalize(currentPath.value)
  navHistory.push(base)
  state.query.path = base === '/' ? `/${name}` : `${base}/${name}`
  state.page = 1
  loadImages()
}

function navigateTo(full: string) {
  const base = normalize(currentPath.value)
  if (full === base) return
  navHistory.push(base)
  state.query.path = normalize(full)
  state.page = 1
  loadImages()
}

function goUp() {
  const base = normalize(currentPath.value)
  if (base === '/') return
  const parent = base.split('/').slice(0, -1).join('/') || '/'
  navHistory.push(base)
  state.query.path = parent
  state.page = 1
  loadImages()
}

function goBack() {
  if (!navHistory.length) return
  const prev = navHistory.pop()!
  state.query.path = prev
  state.page = 1
  loadImages()
}

function selectTile(_payload: any) {
  // Placeholder for selection state if needed later
}

async function copy(text: string) {
  try { await navigator.clipboard.writeText(text) } catch {}
}
</script>

<style scoped></style>