<script setup>
import { RouterLink, RouterView } from 'vue-router'
</script>

<template>
  <div class="min-h-screen bg-gray-100">
    <nav class="bg-white shadow-lg">
      <div class="max-w-7xl mx-auto px-4">
        <div class="flex justify-between">
          <div class="flex space-x-7">
            <div>
              <a href="#" class="flex items-center py-4 px-2">
                <span class="font-semibold text-gray-500 text-lg">도서 관리 시스템</span>
              </a>
            </div>
          </div>
          <div class="hidden md:flex items-center space-x-3">
            <RouterLink to="/" class="py-2 px-2 font-medium text-gray-500 rounded hover:bg-green-500 hover:text-white transition duration-300">도서 목록</RouterLink>
            <RouterLink to="/add" class="py-2 px-2 font-medium text-white bg-green-500 rounded hover:bg-green-400 transition duration-300">새 도서 추가</RouterLink>
          </div>
        </div>
      </div>
    </nav>

    <div class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
      <RouterView></RouterView>
    </div>
  </div>
</template>
