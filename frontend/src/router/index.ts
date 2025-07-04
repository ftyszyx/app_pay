import { createRouter, createWebHistory } from 'vue-router'
import AdminLayout from '@/layouts/AdminLayout.vue'
import DashboardView from '@/views/admin/DashboardView.vue'
import ProductAdminView from '@/views/admin/ProductAdminView.vue'
import OrderAdminView from '@/views/admin/OrderAdminView.vue'
import UserAdminView from '@/views/admin/UserAdminView.vue'
import LoginView from '@/views/admin/LoginView.vue'
import { useAuthStore } from '@/stores/auth'

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: '/',
            name: 'home',
            component: () => import('@/views/HomeView.vue')
        },
        {
            path: '/products',
            name: 'products',
            component: () => import('@/views/ProductsView.vue')
        },
        {
            path: '/login',
            name: 'login',
            component: LoginView
        },
        {
            path: '/admin',
            component: AdminLayout,
            meta: { requiresAuth: true },
            redirect: '/admin/dashboard',
            children: [
                {
                    path: 'dashboard',
                    name: 'admin-dashboard',
                    component: DashboardView
                },
                {
                    path: 'products',
                    name: 'admin-products',
                    component: ProductAdminView
                },
                {
                    path: 'orders',
                    name: 'admin-orders',
                    component: OrderAdminView
                },
                {
                    path: 'users',
                    name: 'admin-users',
                    component: UserAdminView
                }
            ]
        }
    ]
})

router.beforeEach((to, _, next) => {
    const authStore = useAuthStore()
    if (to.meta.requiresAuth && !authStore.isAuthenticated) {
        next({ name: 'login' })
    } else {
        next()
    }
})

export default router 