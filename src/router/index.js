import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'books',
      component: () => import('../views/BookList.vue')
    },
    {
      path: '/add',
      name: 'add-book',
      component: () => import('../views/AddBook.vue')
    },
    {
      path: '/edit/:id',
      name: 'edit-book',
      component: () => import('../views/EditBook.vue')
    },
    {
      path: '/book/:id',
      name: 'book-detail',
      component: () => import('../views/BookDetail.vue')
    }
  ]
})

export default router