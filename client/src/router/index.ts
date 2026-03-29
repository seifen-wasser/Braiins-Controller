import { createRouter, createWebHistory } from 'vue-router'

/*const routes = [
  {
    path: '/',
    name: 'Dashboard',
    component: () => import('../views/Dashboard.vue')
  },
  {
    path: '/shutdown',
    name: 'Shutdown',
    component: () => import('../views/Shutdown.vue')
  },
  {
    path: '/settings',
    name: 'Settings',
    component: () => import('../views/Settings.vue')
  },
  {
    path: '/logs',
    name: 'Logs',
    component: () => import('../views/Logs.vue')
  }
]*/

const routes = [
    {
        path: '/',
        name: 'Dashboard',
        component: () => import("../views/Dashboard.vue")
    }
];

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router