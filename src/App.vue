<script setup lang="ts">
import { ref } from 'vue';
import { RouterView, RouterLink, useRoute } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';

const route = useRoute();

const isActive = (path: string) => route.path === path;

const isAuthenticated = ref(false);
const passwordInput = ref('');
const loginError = ref('');

async function handleLogin() {
  if (!passwordInput.value) {
    loginError.value = '请输入密码';
    return;
  }
  
  try {
    const isValid = await invoke<boolean>('verify_password', { password: passwordInput.value });
    if (isValid) {
      isAuthenticated.value = true;
      loginError.value = '';
    } else {
      loginError.value = '密码错误';
    }
  } catch (e) {
    console.error('Login failed', e);
    loginError.value = '验证出错: ' + e;
  }
}
</script>

<template>
  <div v-if="!isAuthenticated" class="min-h-screen flex items-center justify-center bg-gray-50">
      <div class="bg-white p-8 rounded-lg shadow-lg w-96">
        <h2 class="text-2xl font-bold mb-6 text-center text-gray-800">请输入启动密码</h2>
        <input 
          v-model="passwordInput" 
          type="password" 
          placeholder="密码" 
          class="w-full border border-gray-300 rounded px-4 py-2 mb-4 focus:outline-none focus:ring-2 focus:ring-blue-500"
          @keyup.enter="handleLogin"
        />
        <div v-if="loginError" class="text-red-500 text-sm mb-4">{{ loginError }}</div>
        <button 
          @click="handleLogin" 
          class="w-full bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded transition-colors"
        >
          进入应用
        </button>
      </div>
  </div>
  <div v-else class="min-h-screen bg-gray-50 text-gray-900 font-sans selection:bg-blue-200 selection:text-blue-900 flex flex-col">
    <!-- 顶部导航栏 -->
    <nav class="bg-white/80 backdrop-blur-md border-b border-gray-200 sticky top-0 z-40 shadow-sm">
      <div class="container mx-auto px-6 h-16 flex items-center justify-between">
        <div class="flex items-center gap-8">
          <span class="text-xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-blue-600 to-purple-600">
            AI Wallpaper
          </span>
          <div class="flex gap-1">
            <RouterLink 
              to="/" 
              class="px-4 py-2 rounded-lg text-sm font-medium transition-all"
              :class="isActive('/') ? 'bg-blue-50 text-blue-600 shadow-sm ring-1 ring-blue-100' : 'text-gray-600 hover:text-gray-900 hover:bg-gray-100'"
            >
              生成
            </RouterLink>
            <RouterLink 
              to="/gallery" 
              class="px-4 py-2 rounded-lg text-sm font-medium transition-all"
              :class="isActive('/gallery') ? 'bg-blue-50 text-blue-600 shadow-sm ring-1 ring-blue-100' : 'text-gray-600 hover:text-gray-900 hover:bg-gray-100'"
            >
              画廊
            </RouterLink>
            <RouterLink 
              to="/prompt" 
              class="px-4 py-2 rounded-lg text-sm font-medium transition-all"
              :class="isActive('/prompt') ? 'bg-blue-50 text-blue-600 shadow-sm ring-1 ring-blue-100' : 'text-gray-600 hover:text-gray-900 hover:bg-gray-100'"
            >
              提示词库
            </RouterLink>
            <RouterLink 
              to="/settings" 
              class="px-4 py-2 rounded-lg text-sm font-medium transition-all"
              :class="isActive('/settings') ? 'bg-blue-50 text-blue-600 shadow-sm ring-1 ring-blue-100' : 'text-gray-600 hover:text-gray-900 hover:bg-gray-100'"
            >
              设置
            </RouterLink>
          </div>
        </div>
      </div>
    </nav>

    <!-- 主内容区 -->
    <main class="flex-1 container mx-auto px-6 py-8 max-w-4xl">
      <RouterView v-slot="{ Component }">
        <transition name="fade" mode="out-in">
          <component :is="Component" />
        </transition>
      </RouterView>
    </main>
  </div>
</template>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
