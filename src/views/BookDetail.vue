<template>
  <div v-if="book" class="max-w-4xl mx-auto p-6 bg-white shadow-lg rounded-lg mt-10 flex flex-col">
    <h1 class="text-3xl font-bold mb-6 text-gray-800">{{ book.title }}</h1>
    <div class="flex flex-col md:flex-row flex-grow">
      <div class="flex-1 pr-0 md:pr-6">
        <div class="mb-4">
          <p class="text-gray-600"><span class="font-semibold w-32 inline-block">저자:</span> {{ book.author }}</p>
          <p v-if="book.translator" class="text-gray-600"><span class="font-semibold w-32 inline-block">번역자:</span> {{ book.translator }}</p>
          <p v-if="book.publisher" class="text-gray-600"><span class="font-semibold w-32 inline-block">출판사:</span> {{ book.publisher }}</p>
          <p v-if="book.isbn" class="text-gray-600"><span class="font-semibold w-32 inline-block">ISBN:</span> {{ book.isbn }}</p>
          <p v-if="book.purchasedate" class="text-gray-600"><span class="font-semibold w-32 inline-block">구입일:</span> {{ book.purchasedate }}</p>
          <p v-if="book.purchaselocation" class="text-gray-600"><span class="font-semibold w-32 inline-block">구입처:</span> {{ book.purchaselocation }}</p>
        </div>
      </div>
      <div v-if="book.coverimage" class="mt-4">
        <img :src="coverImageUrl" alt="Book cover" class="max-w-xs rounded-lg shadow-md">
      </div>
    </div>
    <div class="flex justify-end space-x-4 mt-6 pt-4 border-t border-gray-200">
      <RouterLink 
        :to="{ name: 'edit-book', params: { id: book.id } }" 
        class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition duration-300"
      >
        편집
      </RouterLink>
      <button 
        @click="showDeleteConfirm = true"
        class="px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600 transition duration-300"
      >
        삭제
      </button>
    </div>

    <!-- 삭제 확인 대화 상자 -->
    <div v-if="showDeleteConfirm" class="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full flex items-center justify-center">
      <div class="bg-white p-5 rounded-lg shadow-xl max-w-sm mx-auto">
        <h3 class="text-lg font-bold mb-4">삭제 확인</h3>
        <p class="mb-4">정말로 이 책을 삭제하시겠습니까?</p>
        <div class="flex justify-end space-x-3">
          <button 
            @click="showDeleteConfirm = false"
            class="px-4 py-2 bg-gray-300 text-gray-800 rounded hover:bg-gray-400 transition duration-300"
          >
            취소
          </button>
          <button 
            @click="deleteBook"
            class="px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600 transition duration-300"
          >
            삭제
          </button>
        </div>
      </div>
    </div>
    <!-- // 삭제 확인 -->
  </div>
</template>

<script>
// 스크립트 부분은 이전과 동일합니다
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useBookStore } from '../stores/bookStore'
import { convertFileSrc } from '@tauri-apps/api/core'
import { appDataDir, join } from '@tauri-apps/api/path'
import { remove } from '@tauri-apps/plugin-fs';

export default {
  setup() {
    const route = useRoute()
    const router = useRouter()
    const bookStore = useBookStore()
    const book = ref(null)
    const showDeleteConfirm = ref(false)
    const coverImageUrl = ref('')

    onMounted(async () => {
      const id = parseInt(route.params.id)
      book.value = await bookStore.getBook(id)
      if (book.value && book.value.coverimage) {
        const appDataDirPath = await appDataDir()
        const imagePath = await join(appDataDirPath, book.value.coverimage)
        //coverImageUrl.value = 'file://' + imagePath
        coverImageUrl.value = convertFileSrc(imagePath)
      }
    })

    const deleteBook = async () => {
      try {
        if (book.value && book.value.coverimage) {
          const appDataDirPath = await appDataDir()
          const imagePath = await join(appDataDirPath, book.value.coverimage)
          await remove(imagePath);
        }

        await bookStore.deleteBook(book.value.id)
        router.push('/')
      } catch (error) {
        console.error('Failed to delete book:', error)
        alert('책 삭제에 실패했습니다. 다시 시도해 주세요.')
      } finally {
        showDeleteConfirm.value = false
      }
    }

    return {
      book,
      showDeleteConfirm,
      deleteBook,
      coverImageUrl
    }
  }
}
</script>