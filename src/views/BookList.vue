<template>
  <div class="bg-white shadow overflow-hidden sm:rounded-lg">
    <div class="px-4 py-5 sm:px-6">
      <h1 class="text-3xl font-bold leading-tight text-gray-900">도서 목록</h1>
    </div>
    <ul class="divide-y divide-gray-200">
      <li v-for="book in bookStore.books" :key="book.uuid" class="px-4 py-4 sm:px-6">
        <div class="flex items-center justify-between">
          <div class="flex items-center">
            <div class="ml-4">
              <div class="text-lg font-semibold text-gray-900">
                <RouterLink :to="{ name: 'book-detail', params: { id: book.id } }">
                    {{ book.title }}
                </RouterLink>
              </div>
              <div class="text-sm text-gray-500">
                {{ book.author }}
              </div>
            </div>
          </div>
        </div>
      </li>
    </ul>
  </div>
</template>

<script>
import { onMounted } from 'vue'
import { useBookStore } from '../stores/bookStore'
import { RouterLink } from 'vue-router'

export default {
  components: {
    RouterLink
  },
  setup() {
    const bookStore = useBookStore()

    onMounted(() => {
      bookStore.fetchBooks()
    })

    return {
      bookStore
    }
  }
}
</script>