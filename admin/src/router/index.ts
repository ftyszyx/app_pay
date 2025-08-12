import { createRouter, createWebHistory } from 'vue-router'
import AdminLayout from '@/layouts/AdminLayout.vue'
import DashboardView from '@/views/admin/DashboardView.vue'
import ProductAdminView from '@/views/admin/ProductAdminView.vue'
import OrderAdminView from '@/views/admin/OrderAdminView.vue'
import UserAdminView from '@/views/admin/UserAdminView.vue'
import AppAdminView from '@/views/admin/AppAdminView.vue'
import ImageAdminView from '@/views/admin/ImageAdminView.vue'
import LoginView from '@/views/auth/LoginView.vue'
import RegisterView from '@/views/auth/RegisterView.vue'
import { useAuthStore } from '@/stores/auth'
import { RouteName, RoutePath } from '@/types'

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: '/',
            component: () => import('@/layouts/HomeLayout.vue'),
            children: [
                {
                    path: '',
                    name: RouteName.Home,
                    component: () => import('@/views/HomeView.vue')
                },
                {
                    path: RoutePath.Products,
                    name: RouteName.Products,
                    component: () => import('@/views/ProductsView.vue')
                }
            ]
        },
        {
            path: RoutePath.Login,
            name: RouteName.Login,
            component: LoginView
        },
        {
            path: RoutePath.Register,
            name: RouteName.Register,
            component: RegisterView
        },
        {
            path: RoutePath.Admin,
            component: AdminLayout,
            meta: { requiresAuth: true },
            redirect: RoutePath.AdminDashboard,
            children: [
                {
                    path: RoutePath.AdminDashboard,
                    name: RouteName.AdminDashboard,
                    component: DashboardView
                },
                {
                    path: RoutePath.AdminProducts,
                    name: RouteName.AdminProducts,
                    component: ProductAdminView
                },
                {
                    path: RoutePath.AdminOrders,
                    name: RouteName.AdminOrders,
                    component: OrderAdminView
                },
                {
                    path: RoutePath.AdminUsers,
                    name: RouteName.AdminUsers,
                    component: UserAdminView
                },
                {
                    path: RoutePath.AdminApps,
                    name: RouteName.AdminApps,
                    component: AppAdminView
                },
                {
                    path: RoutePath.AdminImages,
                    name: RouteName.AdminImages,
                    component: ImageAdminView
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