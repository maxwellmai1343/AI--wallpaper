<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { convertFileSrc } from "@tauri-apps/api/core";
import { WallpaperRecord } from "./types";

const prompt = ref("");
const history = ref<WallpaperRecord[]>([]);
const loading = ref(false);
const currentResolution = ref({ width: 1920, height: 1080 });

async function fetchHistory() {
  try {
    history.value = await invoke("get_wallpaper_history");
  } catch (e) {
    console.error("获取历史记录失败", e);
  }
}

async function fetchResolution() {
  try {
    const res = await invoke<[number, number]>("get_screen_resolution");
    currentResolution.value = { width: res[0], height: res[1] };
  } catch (e) {
    console.error("获取屏幕分辨率失败", e);
  }
}

async function generate() {
  if (!prompt.value) return;
  loading.value = true;
  try {
    console.log("Calling generate_wallpaper...");
    const imagePath = await invoke<string>("generate_wallpaper", {
      prompt: prompt.value,
      width: currentResolution.value.width,
      height: currentResolution.value.height,
    });
    console.log("Generated image path:", imagePath);
    
    console.log("Calling setWallpaper...");
    await setWallpaper(imagePath);
    
    console.log("Calling fetchHistory...");
    await fetchHistory();
    // 简单的成功提示，实际项目中可以使用更优雅的 Toast 组件
    // alert("壁纸设置成功！"); 
  } catch (e) {
    console.error("生成失败", e);
    alert("生成失败: " + e);
  } finally {
    loading.value = false;
  }
}

async function setWallpaper(path: string) {
  try {
    await invoke("set_wallpaper", { imagePath: path });
  } catch (e) {
    console.error("设置壁纸失败", e);
    throw e; // 抛出错误以便在上层捕获
  }
}

async function clearCache() {
  if (confirm("确定要清空历史记录和缓存吗？")) {
    await invoke("clear_cache");
    history.value = [];
  }
}

function getImageUrl(path: string) {
  return convertFileSrc(path);
}

onMounted(() => {
  fetchResolution();
  fetchHistory();
});
</script>

<template>
  <div class="container mx-auto p-4 max-w-2xl bg-gray-900 text-white min-h-screen">
    <h1 class="text-2xl font-bold mb-4">AI 壁纸生成器</h1>
    
    <div class="mb-6 bg-gray-800 p-4 rounded-lg">
      <div class="flex justify-between items-center mb-2 text-sm text-gray-400">
        <span>屏幕分辨率: {{ currentResolution.width }}x{{ currentResolution.height }}</span>
      </div>
      
      <textarea
        v-model="prompt"
        class="w-full bg-gray-700 border border-gray-600 rounded p-2 text-white mb-4 focus:outline-none focus:border-blue-500"
        rows="3"
        placeholder="在此输入提示词 (例如：'霓虹灯下的未来城市')..."
      ></textarea>
      
      <button
        @click="generate"
        :disabled="loading || !prompt"
        class="w-full bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
      >
        {{ loading ? "生成中..." : "生成并设置壁纸" }}
      </button>
    </div>

    <div v-if="history.length > 0" class="bg-gray-800 p-4 rounded-lg">
      <div class="flex justify-between items-center mb-4">
        <h2 class="text-xl font-semibold">历史记录</h2>
        <button @click="clearCache" class="text-xs text-red-400 hover:text-red-300">清空历史</button>
      </div>
      
      <div class="grid grid-cols-2 gap-4">
        <div v-for="item in history.slice().reverse()" :key="item.id" class="relative group">
          <div class="aspect-video bg-gray-700 rounded overflow-hidden cursor-pointer" @click="setWallpaper(item.imagePath)">
             <img :src="getImageUrl(item.imagePath)" class="w-full h-full object-cover hover:opacity-80 transition-opacity" alt="壁纸预览" />
             <div class="absolute inset-0 flex items-center justify-center opacity-0 group-hover:opacity-100 bg-black bg-opacity-50 transition-opacity">
               <span class="text-white font-bold">设为壁纸</span>
             </div>
          </div>
          <div class="mt-1 text-xs truncate text-gray-400" :title="item.prompt">
            {{ item.prompt }}
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
</style>
