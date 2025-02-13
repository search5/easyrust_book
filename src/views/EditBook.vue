<template>
  <div class="bg-white shadow overflow-hidden sm:rounded-lg">
    <div class="px-4 py-5 sm:px-6">
      <h1 class="text-3xl font-bold leading-tight text-gray-900">도서 수정</h1>
    </div>
    <div class="px-4 py-5 sm:p-6">
      <form @submit.prevent="updateBook" class="space-y-4">
        <div class="flex flex-col space-y-4">
          <div class="flex items-center">
            <label for="title" class="block text-sm font-medium text-gray-700 w-1/4">제목</label>
            <input type="text" id="title" v-model="book.title" required
                   class="mt-1 block w-3/4 border border-gray-300 rounded-md shadow-sm py-2 px-3 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm">
          </div>
          <div class="flex items-center">
            <label for="author" class="block text-sm font-medium text-gray-700 w-1/4">저자</label>
            <input type="text" id="author" v-model="book.author" required
                   class="mt-1 block w-3/4 border border-gray-300 rounded-md shadow-sm py-2 px-3 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm">
          </div>
          <div class="flex items-center">
            <label for="translator" class="block text-sm font-medium text-gray-700 w-1/4">번역자</label>
            <input type="text" id="translator" v-model="book.translator"
                   class="mt-1 block w-3/4 border border-gray-300 rounded-md shadow-sm py-2 px-3 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm">
          </div>
          <div class="flex items-center">
            <label for="publisher" class="block text-sm font-medium text-gray-700 w-1/4">출판사</label>
            <input type="text" id="publisher" v-model="book.publisher"
                   class="mt-1 block w-3/4 border border-gray-300 rounded-md shadow-sm py-2 px-3 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm">
          </div>
          <div class="flex items-center">
            <label for="purchaseDate" class="block text-sm font-medium text-gray-700 w-1/4">구입일</label>
            <input type="date" id="purchaseDate" v-model="book.purchasedate"
                   class="mt-1 block w-3/4 border border-gray-300 rounded-md shadow-sm py-2 px-3 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm">
          </div>
          <div class="flex items-center">
            <label for="purchaseLocation" class="block text-sm font-medium text-gray-700 w-1/4">구입처</label>
            <input type="text" id="purchaseLocation" v-model="book.purchaselocation"
                   class="mt-1 block w-3/4 border border-gray-300 rounded-md shadow-sm py-2 px-3 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm">
          </div>
          <div class="flex items-center">
            <label for="isbn" class="block text-sm font-medium text-gray-700 w-1/4">ISBN</label>
            <input type="text" id="isbn" v-model="book.isbn"
                   class="mt-1 block w-3/4 border border-gray-300 rounded-md shadow-sm py-2 px-3 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm">
          </div>
        </div>
        <div class="space-y-4">
          <div class="flex items-start">
            <label class="text-sm font-medium text-gray-700 mb-1 w-1/4">도서 커버</label>
            <div class="w-3/4">
              <div class="flex items-center justify-between mb-2">
                <span class="text-sm text-gray-500 truncate mr-2 flex-1">{{ selectedFileName || '선택된 파일 없음' }}</span>
                <button type="button" @click="openFileDialog" 
                        class="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 transition duration-300 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2">
                  파일 선택
                </button>
              </div>
              <div v-if="imagePreview" class="mt-2 border-2 border-gray-300 rounded-lg p-2">
                <img :src="imagePreview" alt="Book cover preview" class="max-w-full h-auto rounded-lg shadow-md">
              </div>
            </div>
          </div>
        </div>
        <div class="mt-6">
          <button type="submit"
                  class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
            도서 수정
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script>
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useBookStore } from '../stores/bookStore'
import { open } from '@tauri-apps/plugin-dialog'
import { readFile, writeFile, mkdir } from '@tauri-apps/plugin-fs'
import { appDataDir, join, basename } from '@tauri-apps/api/path'
import { convertFileSrc } from '@tauri-apps/api/core'

export default {
  setup() {
    const route = useRoute()
    const router = useRouter()
    const bookStore = useBookStore()
    const book = ref({})  // 빈 객체로 초기화

    const selectedFileName = ref('')
    const imagePreview = ref('')
    const selectedFileContents = ref(null)

    const openFileDialog = async () => {
      try {
        const selected = await open({
          multiple: false,
          filters: [{
            name: 'Image',
            extensions: ['jpg', 'jpeg', 'png', 'gif', 'avif']
          }]
        })
        if (selected) {
          selectedFileName.value = await basename(selected)
          
          // 이미지 미리보기 생성
          const contents = await readFile(selected)
          selectedFileContents.value = contents
          const blob = new Blob([contents], { type: 'image/jpeg' })
          imagePreview.value = URL.createObjectURL(blob)
        }
      } catch (err) {
        console.error('Failed to open file dialog:', err)
      }
    }

    onMounted(async () => {
      const id = route.params.id
      if (id) {
        const fetchedBook = await bookStore.getBook(parseInt(id))
        fetchedBook.id = parseInt(id)
        book.value = fetchedBook || {}  // fetched book이 없으면 빈 객체 유지
        // 이미지가 선택되어 있으면 미리보기를 보여준다.
        if (book.value.coverimage != "") {
          const appDataDirPath = await appDataDir()
          const imagePath = await join(appDataDirPath, book.value.coverimage)
          imagePreview.value = convertFileSrc(imagePath)
          selectedFileName.value = await basename(book.value.coverimage)
        }
      }
    })

    const updateBook = async () => {
      try {
        if (selectedFileContents.value) {
          const appDataDirPath = await appDataDir();
          const bookCoversDir = await join(appDataDirPath, 'book_covers');
          await mkdir(bookCoversDir, { recursive: true });
          const fileName = `${Date.now()}_${selectedFileName.value}`;
          const relativeFilePath = await join('book_covers', fileName);
          const fullFilePath = await join(bookCoversDir, fileName);

          await writeFile(fullFilePath, selectedFileContents.value);

          book.value.coverimage = relativeFilePath;
        }

        await bookStore.updateBook(book.value)
        router.push('/')
      } catch (error) {
        console.error('Failed to add book:', error)
      }
    }

    return {
      book,
      updateBook,
      openFileDialog,
      selectedFileName,
      imagePreview
    }
  }
}
</script>