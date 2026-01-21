<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const apiKey = ref('');
const model = ref('black-forest-labs/FLUX-2-klein-4b:free');
const storagePath = ref('');
const showSuccess = ref(false);

onMounted(async () => {
  const storedKey = localStorage.getItem('ai_api_key');
  const storedModel = localStorage.getItem('ai_model');
  if (storedKey) apiKey.value = storedKey;
  if (storedModel) model.value = storedModel;
  
  try {
    storagePath.value = await invoke('get_storage_path');
  } catch (e) {
    console.error('获取存储路径失败', e);
  }
});

async function saveSettings() {
  localStorage.setItem('ai_api_key', apiKey.value);
  localStorage.setItem('ai_model', model.value);
  
  try {
    await invoke('set_storage_path', { path: storagePath.value });
    showSuccess.value = true;
    setTimeout(() => showSuccess.value = false, 2000);
  } catch (e) {
    alert('保存存储路径失败: ' + e);
  }
}
</script>

<template>
  <div class="animate-fade-in-up">
    <!-- 头部区域 -->
    <header class="mb-8 border-b border-gray-200 pb-6">
      <h2 class="text-3xl font-bold text-gray-800 flex items-center gap-3">
        <span class="text-blue-500">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
          </svg>
        </span>
        设置
      </h2>
      <p class="text-gray-500 text-sm mt-1">配置 API 密钥和模型参数</p>
    </header>

    <!-- 设置表单 -->
    <div class="bg-white border border-gray-200 rounded-xl p-6 shadow-sm">
      <div class="space-y-6">
        <!-- API Key -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">API Key</label>
          <div class="relative">
            <input
              v-model="apiKey"
              type="password"
              placeholder="sk-..."
              class="w-full bg-gray-50 border border-gray-300 rounded-lg py-3 px-4 text-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500/50 focus:border-blue-500/50 transition-all font-mono"
            />
          </div>
          <p class="text-xs text-gray-500 mt-1">请输入您的 SiliconFlow 或兼容的 OpenAI API 密钥</p>
        </div>

        <!-- Model Selection -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">模型 (Model)</label>
          <input
            v-model="model"
            type="text"
            class="w-full bg-gray-50 border border-gray-300 rounded-lg py-3 px-4 text-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500/50 focus:border-blue-500/50 transition-all font-mono"
          />
          <p class="text-xs text-gray-500 mt-1">默认: black-forest-labs/FLUX-2-klein-4b:free</p>
        </div>

        <!-- Storage Path -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">图片存储路径</label>
          <input
            v-model="storagePath"
            type="text"
            class="w-full bg-gray-50 border border-gray-300 rounded-lg py-3 px-4 text-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500/50 focus:border-blue-500/50 transition-all font-mono"
          />
          <p class="text-xs text-gray-500 mt-1">设置图片和历史记录的保存位置</p>
        </div>

        <!-- Save Button -->
        <div class="pt-4 flex items-center gap-4">
          <button
            @click="saveSettings"
            class="bg-blue-600 hover:bg-blue-700 text-white font-medium py-2.5 px-6 rounded-lg transition-colors shadow-sm flex items-center gap-2"
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
            </svg>
            保存配置
          </button>
          
          <transition name="fade">
            <span v-if="showSuccess" class="text-green-600 flex items-center gap-1 text-sm font-medium">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              保存成功
            </span>
          </transition>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>