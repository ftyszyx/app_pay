<template>
  <div class="p-4 md:p-6 space-y-4">
    <div class="flex flex-wrap items-center gap-2">
      <div class="flex items-center gap-2">
        <button
          class="border border-gray-300 rounded px-2 py-1 text-sm bg-white hover:bg-gray-50 disabled:opacity-50"
          @click="goBack"
          title="Back"
          :disabled="!canGoBack"
        >
          ←
        </button>
        <button
          class="border border-gray-300 rounded px-2 py-1 text-sm bg-white hover:bg-gray-50 disabled:opacity-50"
          @click="goUp"
          title="Up"
          :disabled="!canGoUp"
        >
          ↑
        </button>
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
        <button class="px-3 py-1.5 rounded text-white bg-blue-600 hover:bg-blue-700 text-sm" @click="load">Search</button>
        <button class="px-3 py-1.5 rounded border border-gray-300 text-sm" @click="refresh">Refresh</button>
        <button class="px-3 py-1.5 rounded text-white bg-green-600 hover:bg-green-700 text-sm" @click="openCreateDialog">New</button>
        <div class="h-5 w-px bg-gray-200 mx-1" />
        <div class="flex items-center gap-1">
          <button class="px-2 py-1 text-sm border rounded" :class="viewMode==='grid'?'bg-gray-100':''" @click="viewMode='grid'" title="Grid view">▦</button>
          <button class="px-2 py-1 text-sm border rounded" :class="viewMode==='list'?'bg-gray-100':''" @click="viewMode='list'" title="List view">☰</button>
          <button class="px-2 py-1 text-sm border rounded" :class="viewMode==='vf'?'bg-gray-100':''" @click="viewMode='vf'" title="VueFinder">VF</button>
        </div>
        <div class="flex items-center gap-1 text-sm">
          <span class="text-gray-500">Sort</span>
          <select v-model="sortKey" class="border rounded px-1 py-0.5">
            <option value="name">Name</option>
            <option value="type">Type</option>
            <option value="path">Path</option>
            <option value="updated_at">Modified</option>
          </select>
          <button class="px-2 py-1 border rounded" @click="toggleOrder" :title="sortOrder==='asc'?'Ascending':'Descending'">{{ sortOrder==='asc'?'↑':'↓' }}</button>
        </div>
      </div>
    </div>

    <div class="border rounded bg-white overflow-hidden">
      <div class="p-2 border-b text-xs text-gray-500 bg-gray-50">{{ currentPath || "/" }}</div>
      <div v-if="viewMode==='vf'" class="p-2">
        <vue-finder id="vf" :request="vfRequest" theme="light" max-height="70vh" />
      </div>
      <div v-else-if="viewMode==='grid'" class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 xl:grid-cols-8 gap-3 p-3">
        <div v-for="folder in folders" :key="folder" class="rounded p-2 hover:bg-blue-50 cursor-default group" @dblclick="openFolder(folder)">
          <div class="w-full aspect-[4/3] bg-yellow-50 rounded relative overflow-hidden flex items-center justify-center">
            <svg viewBox="0 0 24 24" class="w-10 h-10 text-yellow-500">
              <path fill="currentColor" d="M10 4H4a2 2 0 0 0-2 2v2h20V8a2 2 0 0 0-2-2h-8l-2-2Z" />
              <path fill="currentColor" d="M22 10H2v8a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2Z" />
            </svg>
          </div>
          <div class="mt-2 text-xs text-gray-800 truncate" :title="folder">{{ folder }}</div>
        </div>

        <div v-for="it in files" :key="it.id" class="rounded p-2 hover:bg-blue-50 cursor-default group" @dblclick="edit(it)">
          <div class="w-full aspect-[4/3] bg-gray-100 rounded relative overflow-hidden flex items-center justify-center">
            <img :src="it.url" :alt="it.name" class="w-full h-full object-cover" />
            <div
              class="absolute inset-0 flex items-end justify-center gap-2 p-2 opacity-0 transition-opacity bg-gradient-to-t from-black/50 to-transparent group-hover:opacity-100"
            >
              <button class="text-xs px-2 py-0.5 border border-white/70 rounded bg-black/40 text-white" @click.stop="copy(it.url)">Copy URL</button>
              <button class="text-xs px-2 py-0.5 border border-white/70 rounded bg-black/40 text-white" @click.stop="edit(it)">Edit</button>
              <button class="text-xs px-2 py-0.5 border border-white/70 rounded bg-red-600 text-white" @click.stop="del(it.id)">Delete</button>
            </div>
          </div>
          <div class="mt-2 text-xs text-gray-800 truncate" :title="it.name">{{ it.name }}</div>
        </div>
      </div>
      <div v-else class="p-0">
        <div class="grid grid-cols-[minmax(180px,1.2fr)_140px_minmax(180px,1fr)_180px] text-xs bg-gray-50 border-b px-3 py-2 text-gray-600">
          <div class="cursor-pointer" @click="setSort('name')">Name</div>
          <div class="cursor-pointer" @click="setSort('type')">Type</div>
          <div class="cursor-pointer" @click="setSort('path')">Path</div>
          <div class="cursor-pointer" @click="setSort('updated_at')">Modified</div>
        </div>
        <div v-for="row in sortedEntries" :key="row.key" class="grid grid-cols-[minmax(180px,1.2fr)_140px_minmax(180px,1fr)_180px] items-center text-sm px-3 py-2 hover:bg-blue-50">
          <div class="flex items-center gap-2 truncate">
            <span v-if="row.type==='folder'" class="text-yellow-500">📁</span>
            <img v-else :src="row.thumb" class="w-6 h-6 rounded object-cover" />
            <span class="truncate" :title="row.name">{{ row.name }}</span>
          </div>
          <div class="text-gray-500">{{ row.type==='folder' ? 'Folder' : row.fileType }}</div>
          <div class="text-gray-500 truncate" :title="row.path">{{ row.path }}</div>
          <div class="text-gray-500">{{ row.modified || '-' }}</div>
          <div class="col-span-4 h-px bg-gray-100"></div>
          <div class="col-span-4 -mt-2"></div>
          <div class="col-span-4 hidden"></div>
          <div class="col-span-4 hidden"></div>
          <div class="col-span-4 hidden"></div>
        </div>
        <div class="hidden"></div>
        <div class="hidden"></div>
      </div>
      <div v-if="state.total === 0" class="p-8 text-center text-gray-500">No items</div>
    </div>

    <div class="flex items-center justify-between text-sm">
      <div>Items: {{ state.total }}</div>
      <div class="flex items-center gap-2">
        <button class="px-3 py-1.5 rounded border border-gray-300 text-sm disabled:opacity-50" :disabled="state.page <= 1" @click="prevPage">
          Prev
        </button>
        <span>Page {{ state.page }}</span>
        <button
          class="px-3 py-1.5 rounded border border-gray-300 text-sm disabled:opacity-50"
          :disabled="files.length + folders.length < state.page_size"
          @click="nextPage"
        >
          Next
        </button>
      </div>
    </div>

    <el-dialog v-model="formVisible" :title="form.id ? 'Edit Resource' : 'Create Resource'" width="600">
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
import { reactive, ref, computed, onMounted } from "vue";
import { fetchResources, createResource, updateResource, deleteResource } from "@/apis/resources";
import type { ResourceModel, AddResourceReq, UpdateResourceReq, ListResourcesParams } from "@/types";
import { ElMessage, ElMessageBox, ElInputTag } from "element-plus";
import { uploadFile, genObjectKey } from "@/utils/oss";
import { useOssStore } from "@/stores/oss";

const state = reactive({
  items: [] as ResourceModel[],
  total: 0,
  page: 1,
  page_size: 24,
  query: { path: "/", name: "", object_key: "" } as Partial<ListResourcesParams>,
});

const currentPath = computed(() => state.query.path || "/");

const navHistory: string[] = [];
const canGoBack = computed(() => navHistory.length > 0);
const canGoUp = computed(() => normalize(currentPath.value) !== "/");

const breadcrumb = computed(() => {
  const path = normalize(currentPath.value);
  if (path === "/") return { segments: [] as { name: string; full: string }[] };
  const parts = path.replace(/^\//, "").split("/");
  const segments: { name: string; full: string }[] = [];
  let acc = "";
  for (const p of parts) {
    acc += "/" + p;
    segments.push({ name: p, full: acc });
  }
  return { segments };
});
const viewMode = ref<'grid' | 'list' | 'vf'>('grid');
const vfRequest = {
  baseUrl: (import.meta as any).env?.VITE_API_BASE + '/vuefinder' || '/api/vuefinder',
  headers: { Authorization: 'Bearer ' + localStorage.getItem('token') },
};
const sortKey = ref<'name' | 'type' | 'path' | 'updated_at'>('name');
const sortOrder = ref<'asc' | 'desc'>('asc');
function toggleOrder() { sortOrder.value = sortOrder.value === 'asc' ? 'desc' : 'asc'; }
function setSort(key: 'name' | 'type' | 'path' | 'updated_at') { if (sortKey.value === key) { toggleOrder(); } else { sortKey.value = key; sortOrder.value = 'asc'; } }

async function load() {
  const data = await fetchResources({
    page: state.page,
    page_size: state.page_size,
    path: state.query.path,
    name: state.query.name,
    object_key: state.query.object_key,
  });
  state.items = data.list;
  state.total = data.total;
}

function resetAndLoad() {
  state.page = 1;
  state.query = { path: "/" };
  load();
}
function refresh() {
  load();
}
function prevPage() {
  if (state.page > 1) {
    state.page--;
    load();
  }
}
function nextPage() {
  state.page++;
  load();
}

const formVisible = ref(false);
const tagsList = ref<string[]>([]);
const form = reactive<Partial<ResourceModel & AddResourceReq & UpdateResourceReq>>({
  name: "",
  object_key: "",
  url: "",
  path: "/",
  tags: [],
  status: 1,
  remark: "",
});

function openCreateDialog() {
  Object.assign(form, { id: undefined, name: "", object_key: "", url: "", path: state.query.path || "/", tags: [], status: 1, remark: "" });
  tagsList.value = [];
  formVisible.value = true;
}
function edit(it: ResourceModel) {
  Object.assign(form, it);
  tagsList.value = [...(it.tags || [])];
  formVisible.value = true;
}

async function save() {
  const tags = tagsList.value;
  if (form.id) {
    await updateResource(form.id as number, {
      name: form.name,
      object_key: form.object_key,
      url: form.url,
      path: form.path,
      tags,
      status: form.status,
      remark: form.remark,
    });
  } else {
    await createResource({
      name: form.name as string,
      object_key: form.object_key as string,
      url: form.url as string,
      path: form.path as string,
      tags,
      status: form.status as number,
      remark: form.remark as string | undefined,
    });
  }
  formVisible.value = false;
  load();
}

async function del(id: number) {
  await ElMessageBox.confirm("Delete this resource?");
  await deleteResource(id);
  ElMessage.success("Deleted");
  load();
}

const beforeUpload = (file: File) => {
  const isImage = /^image\//.test(file.type);
  if (!isImage) {
    ElMessage.error("Only image files");
    return false;
  }
  return true;
};
const handleOssUpload = async (options: any) => {
  try {
    const ossStore = useOssStore();
    const client = await ossStore.getClient();
    const key = genObjectKey("uploads/resources", options.file as File);
    const url = await uploadFile(client as any, options.file as File, key);
    form.object_key = key;
    form.url = url;
    ElMessage.success("Uploaded");
    options.onSuccess?.({}, options.file);
  } catch (e) {
    ElMessage.error("Upload failed");
    options.onError?.(e);
  }
};

onMounted(load);

function normalize(path?: string | null) {
  if (!path || path.trim() === "" || path === "/") return "/";
  let p = path.trim();
  if (!p.startsWith("/")) p = "/" + p;
  if (p.length > 1 && p.endsWith("/")) p = p.slice(0, -1);
  return p;
}
const baseWithSlash = computed(() => {
  const base = normalize(currentPath.value);
  return base === "/" ? "/" : base + "/";
});
const folders = computed(() => {
  const set = new Set<string>();
  const base = normalize(currentPath.value);
  const baseSlash = baseWithSlash.value;
  for (const it of state.items) {
    const p = normalize(it.path);
    if (p === base) continue;
    if (base === "/") {
      const seg = p.replace(/^\//, "").split("/")[0];
      if (seg) set.add(seg);
    } else if (p.startsWith(baseSlash)) {
      const rest = p.slice(baseSlash.length);
      const seg = rest.split("/")[0];
      if (seg) set.add(seg);
    }
  }
  return Array.from(set).sort();
});
const files = computed(() => {
  const base = normalize(currentPath.value);
  return state.items.filter((it) => normalize(it.path) === base);
});
const entries = computed(() => {
  const base = normalize(currentPath.value);
  const list: Array<{ key: string; type: 'folder' | 'file'; name: string; path: string; modified?: string | null; thumb?: string; fileType?: string; src?: ResourceModel }> = [];
  for (const folder of folders.value) {
    const fullPath = base === '/' ? `/${folder}` : `${base}/${folder}`;
    list.push({ key: `d-${fullPath}`, type: 'folder', name: folder, path: fullPath, modified: null });
  }
  for (const it of files.value) {
    list.push({ key: `f-${it.id}`, type: 'file', name: it.name, path: normalize(it.path), modified: it.updated_at || it.created_at || null, thumb: it.url, fileType: inferType(it), src: it });
  }
  return list;
});
const sortedEntries = computed(() => {
  const arr = [...entries.value];
  const cmp = (a: any, b: any) => {
    if (a.type !== b.type) return a.type === 'folder' ? -1 : 1;
    let va = '', vb = '';
    switch (sortKey.value) {
      case 'name': va = a.name || ''; vb = b.name || ''; break;
      case 'type': va = a.type === 'folder' ? 'Folder' : (a.fileType || ''); vb = b.type === 'folder' ? 'Folder' : (b.fileType || ''); break;
      case 'path': va = a.path || ''; vb = b.path || ''; break;
      case 'updated_at': va = a.modified || ''; vb = b.modified || ''; break;
    }
    return va.localeCompare(vb);
  };
  arr.sort(cmp);
  if (sortOrder.value === 'desc') arr.reverse();
  return arr;
});
function inferType(it: ResourceModel): string { const key = (it.object_key || it.url || '').toLowerCase(); const m = key.match(/\.([a-z0-9]+)(?:\?|#|$)/); const ext = m?.[1] || ''; if (!ext) return 'File'; const img = ['png','jpg','jpeg','gif','webp','svg','bmp','ico','avif']; if (img.includes(ext)) return ext.toUpperCase() + ' Image'; return ext.toUpperCase() + ' File'; }
function openFolder(name: string) {
  const base = normalize(currentPath.value);
  navHistory.push(base);
  state.query.path = base === "/" ? `/${name}` : `${base}/${name}`;
  state.page = 1;
  load();
}
function navigateTo(full: string) {
  const base = normalize(currentPath.value);
  if (full === base) return;
  navHistory.push(base);
  state.query.path = normalize(full);
  state.page = 1;
  load();
}
function goUp() {
  const base = normalize(currentPath.value);
  if (base === "/") return;
  const parent = base.split("/").slice(0, -1).join("/") || "/";
  navHistory.push(base);
  state.query.path = parent;
  state.page = 1;
  load();
}
function goBack() {
  if (!navHistory.length) return;
  const prev = navHistory.pop()!;
  state.query.path = prev;
  state.page = 1;
  load();
}
async function copy(text: string) {
  try {
    await navigator.clipboard.writeText(text);
  } catch {}
}
</script>

<style scoped></style>
