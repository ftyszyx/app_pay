export enum RouteName {
    Home = 'home',
    Products = 'products',
    Login = 'login',
    Register = 'register',
    Admin = 'admin',
    AdminDashboard = 'dashboard',
    AdminProducts = 'products',
    AdminOrders = 'orders',
    AdminUsers = 'users',
    AdminApps = 'apps',
}


export enum RoutePath {
    Home = '/',
    Products = '/' + RouteName.Products,
    Login = '/' + RouteName.Login,
    Register = '/' + RouteName.Register,
    Admin = '/' + RouteName.Admin,
    AdminDashboard = '/' + RouteName.Admin + '/' + RouteName.AdminDashboard,
    AdminProducts = '/' + RouteName.Admin + '/' + RouteName.AdminProducts,
    AdminOrders = '/' + RouteName.Admin + '/' + RouteName.AdminOrders,
    AdminUsers = '/' + RouteName.Admin + '/' + RouteName.AdminUsers,
    AdminApps = '/' + RouteName.Admin + '/' + RouteName.AdminApps,
}