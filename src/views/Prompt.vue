<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRouter } from 'vue-router';

const router = useRouter();

// 定义 Prompt 和 Category 接口
interface Prompt {
  id: string;
  text: string;
}

interface Category {
  id: string;
  name: string;
  icon: string;
  prompts: Prompt[];
}

const categories = ref<Category[]>([]);

// 默认数据
const defaultCategories: Category[] = [
  {
    id: "nature",
    name: "自然风景",
    icon: "🏔️",
    prompts: [
      { id: "n1", text: "极光下的雪山，湖面倒影，星空璀璨，超高清，8k分辨率" },
    ]
  }
];

const searchQuery = ref("");
const isEditMode = ref(false);
const showEditModal = ref(false);
const editingPrompt = ref<{ categoryId: string, promptId: string | null, text: string }>({ categoryId: '', promptId: null, text: '' });

// 加载数据
onMounted(() => {
  const stored = localStorage.getItem('user_prompts');
  if (stored) {
    try {
      categories.value = JSON.parse(stored);
    } catch (e) {
      console.error("Failed to parse stored prompts", e);
      categories.value = defaultCategories;
    }
  } else {
    categories.value = defaultCategories;
  }
});

// 保存数据
function saveCategories() {
  localStorage.setItem('user_prompts', JSON.stringify(categories.value));
}

// 搜索过滤
const filteredCategories = computed(() => {
  if (!searchQuery.value) return categories.value;
  
  return categories.value.map(category => {
    const filteredPrompts = category.prompts.filter(p => 
      p.text.toLowerCase().includes(searchQuery.value.toLowerCase())
    );
    return {
      ...category,
      prompts: filteredPrompts
    };
  }).filter(category => category.prompts.length > 0);
});

function usePrompt(text: string) {
  if (isEditMode.value) return; // 编辑模式下点击不跳转
  router.push({
    path: '/',
    query: { prompt: text }
  });
}

// 编辑/新建提示词
function openEditModal(categoryId: string, prompt: Prompt | null = null) {
  editingPrompt.value = {
    categoryId,
    promptId: prompt ? prompt.id : null,
    text: prompt ? prompt.text : ''
  };
  showEditModal.value = true;
}

function savePrompt() {
  if (!editingPrompt.value.text.trim()) return;
  
  const category = categories.value.find(c => c.id === editingPrompt.value.categoryId);
  if (!category) return;

  if (editingPrompt.value.promptId) {
    // 编辑现有
    const prompt = category.prompts.find(p => p.id === editingPrompt.value.promptId);
    if (prompt) {
      prompt.text = editingPrompt.value.text;
    }
  } else {
    // 新建
    category.prompts.push({
      id: Date.now().toString(),
      text: editingPrompt.value.text
    });
  }
  
  saveCategories();
  showEditModal.value = false;
}

function deletePrompt(categoryId: string, promptId: string) {
  if (!confirm("确定要删除这条提示词吗？")) return;
  
  const category = categories.value.find(c => c.id === categoryId);
  if (category) {
    category.prompts = category.prompts.filter(p => p.id !== promptId);
    saveCategories();
  }
}

// 恢复默认
function resetDefaults() {
  if (confirm("确定要重置所有提示词吗？您的自定义修改将丢失。")) {
    categories.value = JSON.parse(JSON.stringify(defaultCategories)); // Deep copy
    saveCategories();
  }
}
</script>

<template>
  <div class="animate-fade-in-up pb-20">
    <!-- 头部区域 -->
    <header class="mb-8 border-b border-gray-200 pb-6">
      <div class="flex flex-col md:flex-row md:items-center justify-between gap-4">
        <div>
          <h2 class="text-3xl font-bold text-gray-800 flex items-center gap-3">
            <span class="text-blue-500">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
              </svg>
            </span>
            提示词库
          </h2>
          <p class="text-gray-500 text-sm mt-1">灵感枯竭？试试这些精选提示词</p>
        </div>
        
        <!-- 搜索与操作 -->
        <div class="flex flex-col sm:flex-row gap-3 w-full md:w-auto">
          <div class="relative w-full sm:w-64">
            <input
              v-model="searchQuery"
              type="text"
              placeholder="搜索提示词..."
              class="w-full bg-white border border-gray-300 rounded-lg py-2 px-4 pl-10 text-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500/50 focus:border-blue-500/50 transition-all shadow-sm"
            />
            <svg class="absolute left-3 top-2.5 h-5 w-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
            </svg>
          </div>
          
          <button 
            @click="isEditMode = !isEditMode"
            class="px-4 py-2 rounded-lg text-sm font-medium transition-colors border"
            :class="isEditMode ? 'bg-blue-100 text-blue-700 border-blue-200' : 'bg-white text-gray-600 border-gray-300 hover:bg-gray-50'"
          >
            {{ isEditMode ? '完成编辑' : '管理提示词' }}
          </button>
        </div>
      </div>
    </header>

    <!-- 提示词列表 -->
    <div class="grid grid-cols-1 gap-8">
      <div v-for="category in filteredCategories" :key="category.id" class="space-y-4">
        <div class="flex items-center justify-between sticky top-0 bg-gray-50/90 backdrop-blur py-2 z-10">
          <h3 class="text-xl font-semibold text-gray-800 flex items-center gap-2">
            <span>{{ category.icon }}</span>
            {{ category.name }}
          </h3>
          <button 
            v-if="isEditMode"
            @click="openEditModal(category.id)"
            class="text-sm text-blue-600 hover:text-blue-800 font-medium flex items-center gap-1 bg-blue-50 px-2 py-1 rounded-md"
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
            </svg>
            新增
          </button>
        </div>
        
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div 
            v-for="prompt in category.prompts" 
            :key="prompt.id"
            @click="usePrompt(prompt.text)"
            class="group bg-white border border-gray-200 rounded-xl p-4 transition-all duration-300 relative overflow-hidden"
            :class="isEditMode ? '' : 'hover:bg-blue-50/50 hover:border-blue-300 cursor-pointer hover:shadow-md hover:-translate-y-0.5'"
          >
            <p class="text-gray-600 text-sm leading-relaxed transition-colors pr-8" :class="isEditMode ? '' : 'group-hover:text-gray-900'">
              {{ prompt.text }}
            </p>
            
            <!-- 悬浮图标 (非编辑模式) -->
            <div v-if="!isEditMode" class="absolute right-3 top-3 opacity-0 group-hover:opacity-100 transition-all duration-300 transform translate-x-2 group-hover:translate-x-0">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-blue-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
              </svg>
            </div>

            <!-- 编辑操作栏 (编辑模式) -->
            <div v-if="isEditMode" class="absolute right-2 top-2 flex gap-2">
              <button 
                @click.stop="openEditModal(category.id, prompt)"
                class="p-1.5 text-gray-500 hover:text-blue-600 hover:bg-blue-50 rounded-lg transition-colors"
                title="编辑"
              >
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                </svg>
              </button>
              <button 
                @click.stop="deletePrompt(category.id, prompt.id)"
                class="p-1.5 text-gray-500 hover:text-red-600 hover:bg-red-50 rounded-lg transition-colors"
                title="删除"
              >
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                </svg>
              </button>
            </div>
          </div>
        </div>
      </div>
      
      <!-- 无结果提示 -->
      <div v-if="filteredCategories.length === 0" class="text-center py-12">
        <div class="text-6xl mb-4">🔍</div>
        <p class="text-gray-400">没有找到匹配的提示词</p>
      </div>

      <!-- 底部重置按钮 -->
      <div class="mt-8 pt-8 border-t border-gray-200 text-center">
        <button @click="resetDefaults" class="text-sm text-gray-400 hover:text-gray-600 underline">
          恢复默认提示词
        </button>
      </div>
    </div>

    <!-- 编辑模态框 -->
    <div v-if="showEditModal" class="fixed inset-0 z-50 flex items-center justify-center p-4">
      <div class="absolute inset-0 bg-black/30 backdrop-blur-sm" @click="showEditModal = false"></div>
      <div class="bg-white rounded-xl shadow-2xl w-full max-w-lg relative z-10 overflow-hidden animate-fade-in-up">
        <div class="p-6">
          <h3 class="text-xl font-bold text-gray-800 mb-4">
            {{ editingPrompt.promptId ? '编辑提示词' : '新建提示词' }}
          </h3>
          <textarea
            v-model="editingPrompt.text"
            rows="4"
            class="w-full bg-gray-50 border border-gray-300 rounded-lg p-3 text-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500/50 focus:border-blue-500/50 resize-none"
            placeholder="输入提示词内容..."
          ></textarea>
          <div class="mt-6 flex justify-end gap-3">
            <button 
              @click="showEditModal = false"
              class="px-4 py-2 text-gray-600 hover:bg-gray-100 rounded-lg transition-colors"
            >
              取消
            </button>
            <button 
              @click="savePrompt"
              class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors"
              :disabled="!editingPrompt.text.trim()"
            >
              保存
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* 自定义滚动条样式 */
::-webkit-scrollbar {
  width: 6px;
}
::-webkit-scrollbar-track {
  background: transparent;
}
::-webkit-scrollbar-thumb {
  background: #cbd5e1;
  border-radius: 3px;
}
::-webkit-scrollbar-thumb:hover {
  background: #94a3b8;
}
</style>
