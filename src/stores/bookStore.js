import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'

export const useBookStore = defineStore('book', {
  state: () => ({
    books: []
  }),
  actions: {
    async fetchBooks() {
      try {
        this.books = await invoke('get_books')
      } catch (error) {
        console.error('Failed to fetch books:', error)
      }
    },
    async addBook(book) {
      try {
        await invoke('add_book', book);
        await this.fetchBooks()
      } catch (error) {
        console.error('Failed to add book:', error)
      }
    },
    async getBook(id) {
      try {
        const book = await invoke('get_book', { bookid: id })
        return book
      } catch (error) {
        console.error('Failed to get book:', error)
        throw error
      }
    },
    async updateBook(book) {
      try {
        await invoke('update_book', book)
        await this.fetchBooks()
      } catch (error) {
          console.error('Failed to update book:', error)
      }
    },
    async deleteBook(id) {
      try {
        await invoke('delete_book', { id })
        this.books = this.books.filter(book => book.id !== id)
      } catch (error) {
        console.error('Failed to delete book:', error)
        throw error
      }
    }
  }
})