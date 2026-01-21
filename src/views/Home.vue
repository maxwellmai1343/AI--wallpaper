<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useRoute } from "vue-router";
import { WallpaperRecord } from "../types";

const route = useRoute();
const prompt = ref("");
const loading = ref(false);
const currentResolution = ref({ width: 1920, height: 1080 });

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
    const apiKey = localStorage.getItem('ai_api_key') || "";
    const model = localStorage.getItem('ai_model') || "black-forest-labs/FLUX-2-klein-4b:free";

    console.log("Calling generate_wallpaper...");
    const imagePath = await invoke<string>("generate_wallpaper", {
      prompt: prompt.value,
      width: currentResolution.value.width,
      height: currentResolution.value.height,
      apiKey: apiKey,
      model: model,
    });
    console.log("Generated image path:", imagePath);
    
    console.log("Calling setWallpaper...");
    await setWallpaper(imagePath);
    
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
    throw e;
  }
}

onMounted(() => {
  fetchResolution();
  if (route.query.prompt) {
    prompt.value = route.query.prompt as string;
  }
});
</script>

<template>
  <div class="animate-fade-in-up">
    <!-- 头部区域 -->
    <header class="mb-10 text-center">
      <h1 class="text-4xl font-extrabold text-transparent bg-clip-text bg-gradient-to-r from-blue-600 to-purple-600 mb-2">
        AI 壁纸生成器
      </h1>
      <p class="text-gray-500 text-sm">释放你的创意，一键生成专属桌面</p>
    </header>
    
    <!-- 主操作区 -->
    <div class="bg-white/80 backdrop-blur-sm border border-gray-200 p-6 rounded-2xl shadow-xl mb-10 transition-all hover:border-gray-300">
      <div class="flex justify-between items-center mb-4 text-xs font-medium text-gray-500 uppercase tracking-wider">
        <div class="flex items-center gap-2">
          <span class="w-2 h-2 rounded-full bg-green-500 animate-pulse"></span>
          <span>分辨率: {{ currentResolution.width }} x {{ currentResolution.height }}</span>
        </div>
      </div>
      
      <div class="relative group">
        <textarea
          v-model="prompt"
          class="w-full bg-gray-50 border border-gray-300 rounded-xl p-4 text-gray-900 mb-6 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all resize-none shadow-inner placeholder-gray-400"
          rows="3"
          placeholder="描述你想象中的画面... (例如：'赛博朋克风格的雨夜街道，霓虹灯光反射在湿润的路面上')"
        ></textarea>
        <div class="absolute bottom-8 right-4 text-xs text-gray-400 pointer-events-none group-focus-within:text-blue-500 transition-colors">
          {{ prompt.length }} 字
        </div>
      </div>
      
      <button
        @click="generate"
        :disabled="loading || !prompt"
        class="w-full bg-gradient-to-r from-blue-600 to-purple-600 hover:from-blue-500 hover:to-purple-500 text-white font-bold py-3.5 px-6 rounded-xl disabled:opacity-50 disabled:cursor-not-allowed transition-all transform hover:scale-[1.01] active:scale-[0.99] shadow-lg flex items-center justify-center gap-2"
      >
        <svg v-if="loading" class="animate-spin h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
        </svg>
        <span v-if="!loading">✨ 生成并设置壁纸</span>
        <span v-else>正在绘制梦境...</span>
      </button>
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
