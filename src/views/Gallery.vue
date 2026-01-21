<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { convertFileSrc } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { open } from "@tauri-apps/plugin-dialog";
import { WallpaperRecord } from "../types";

const history = ref<WallpaperRecord[]>([]);
const isDragging = ref(false);
let unlistenDragEnter: (() => void) | null = null;
let unlistenDragLeave: (() => void) | null = null;
let unlistenDragDrop: (() => void) | null = null;

async function fetchHistory() {
  try {
    history.value = await invoke("get_wallpaper_history");
  } catch (e) {
    console.error("获取历史记录失败", e);
  }
}

async function setWallpaper(path: string) {
  try {
    console.log("Calling setWallpaper...");
    await invoke("set_wallpaper", { imagePath: path });
    // alert("壁纸设置成功！");
  } catch (e) {
    console.error("设置壁纸失败", e);
    alert("设置壁纸失败: " + e);
  }
}

async function clearCache() {
  if (confirm("确定要清空历史记录和缓存吗？")) {
    try {
      await invoke("clear_cache");
      history.value = [];
      await fetchHistory(); // Double check
    } catch (e) {
      console.error("清空失败", e);
      alert("清空失败: " + e);
    }
  }
}

function getImageUrl(path: string) {
  // Use custom protocol
  // Need to handle Windows paths and encode properly
  // Example: wallpaper-image://localhost/Users/xiaomai/...
  
  // Normalize path separators (mostly for Windows consistency)
  const normalizedPath = path.replace(/\\/g, '/');
  
  // Encode path segments to handle spaces and special chars
  // Note: We don't want to encode the slashes that are part of the path structure
  const encodedPath = normalizedPath.split('/').map(segment => encodeURIComponent(segment)).join('/');
  
  // Add protocol
  // Note: On Windows it might need another slash if it starts with drive letter, 
  // but our Rust protocol handler handles /C:/...
  return `wallpaper-image://localhost${encodedPath.startsWith('/') ? '' : '/'}${encodedPath}`;
}

function handleImageError(e: Event) {
  const target = e.target as HTMLImageElement;
  target.style.display = 'none'; // Hide broken image
  target.parentElement?.classList.add('bg-red-900'); // Add error indicator to container
  // Add a text indicator
  const span = document.createElement('span');
  span.innerText = '⚠️ 无法加载图片';
  span.className = 'text-red-400 text-xs absolute inset-0 flex items-center justify-center';
  target.parentElement?.appendChild(span);
  console.error('Image load failed:', target.src);
}

onMounted(async () => {
  fetchHistory();
  
  // 监听文件拖放事件
  unlistenDragEnter = await listen("tauri://drag-enter", () => {
    isDragging.value = true;
  });
  
  unlistenDragLeave = await listen("tauri://drag-leave", () => {
    isDragging.value = false;
  });
  
  unlistenDragDrop = await listen("tauri://drag-drop", async (event: any) => {
    isDragging.value = false;
    console.log("Drag drop event:", event);
    if (event.payload.paths && event.payload.paths.length > 0) {
      const filePath = event.payload.paths[0];
      console.log("File path:", filePath);
      // 简单过滤图片文件
      if (/\.(png|jpg|jpeg|webp)$/i.test(filePath)) {
        try {
          await invoke("import_image", { filePath });
          await fetchHistory();
          // alert("图片导入成功！");
        } catch (e) {
          console.error("导入失败", e);
          alert("导入失败: " + e);
        }
      } else {
        alert("请拖入有效的图片文件 (png, jpg, jpeg, webp)");
      }
    } else {
        console.warn("No paths in drag drop event payload", event.payload);
    }
  });
});

onUnmounted(() => {
  if (unlistenDragEnter) unlistenDragEnter();
  if (unlistenDragLeave) unlistenDragLeave();
  if (unlistenDragDrop) unlistenDragDrop();
});
</script>

<template>
  <div class="relative min-h-full">
    <!-- 拖拽遮罩层 -->
    <div v-if="isDragging" class="absolute inset-0 z-50 bg-blue-50/80 backdrop-blur-sm border-4 border-blue-400 border-dashed flex items-center justify-center pointer-events-none rounded-xl">
      <div class="text-2xl font-bold text-blue-600 bg-white/90 px-8 py-4 rounded-xl shadow-lg">
        释放以导入图片到画廊
      </div>
    </div>

    <div class="animate-fade-in-up">
      <div class="flex justify-between items-end mb-6 border-b border-gray-200 pb-2">
        <h2 class="text-2xl font-bold text-gray-800 flex items-center gap-2">
          <svg class="w-6 h-6 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"></path></svg>
          本地画廊
        </h2>
        <div class="flex gap-2">
            <button 
                @click="uploadImage" 
                class="text-xs font-medium bg-blue-50 text-blue-600 hover:bg-blue-100 px-3 py-1.5 rounded-lg transition-colors flex items-center gap-1"
            >
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12" />
                </svg>
                上传图片
            </button>
            <span class="text-xs text-gray-500 self-end mr-2">支持拖拽导入图片</span>
            <button 
                @click="clearCache" 
                class="text-xs font-medium text-red-400 hover:text-red-300 hover:underline px-2 py-1 rounded transition-colors"
            >
                清空所有记录
            </button>
        </div>
      </div>
      
      <div v-if="history.length > 0" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <div v-for="item in history.slice().reverse()" :key="item.id" class="group relative bg-gray-800 rounded-xl overflow-hidden shadow-lg hover:shadow-2xl hover:shadow-purple-500/10 transition-all border border-gray-700 hover:border-purple-500/50">
          <div class="aspect-video relative overflow-hidden cursor-pointer" @click="setWallpaper(item.imagePath)">
             <img 
               :src="getImageUrl(item.imagePath)" 
               @error="handleImageError"
               class="w-full h-full object-cover transform group-hover:scale-110 transition-transform duration-700" 
               alt="壁纸预览" 
             />
             <div class="absolute inset-0 bg-gradient-to-t from-black/80 via-transparent to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300 flex flex-col items-center justify-center">
               <button class="bg-white/10 backdrop-blur-md border border-white/20 text-white px-4 py-2 rounded-full font-medium hover:bg-white/20 transition-colors transform translate-y-4 group-hover:translate-y-0 duration-300">
                 设为壁纸
               </button>
             </div>
          </div>
          <div class="p-3 bg-gray-800 border-t border-gray-700">
            <p class="text-xs text-gray-400 line-clamp-2 leading-relaxed group-hover:text-gray-200 transition-colors" :title="item.prompt">
              {{ item.prompt }}
            </p>
            <div class="mt-2 flex justify-between items-center text-[10px] text-gray-500 font-mono">
              <span>{{ new Date(item.createdAt).toLocaleDateString() }}</span>
              <span>{{ item.resolution.width }}x{{ item.resolution.height }}</span>
            </div>
          </div>
        </div>
      </div>
      
      <!-- 空状态 -->
      <div v-else class="text-center py-20 text-gray-500 border-2 border-dashed border-gray-700 rounded-xl">
        <div class="text-6xl mb-4">🖼️</div>
        <p class="mb-2">画廊空空如也</p>
        <p class="text-sm">你可以去生成壁纸，或者直接拖拽图片到这里导入</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
@keyframes fadeInUp {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
.animate-fade-in-up {
  animation: fadeInUp 0.5s ease-out;
}
</style>
