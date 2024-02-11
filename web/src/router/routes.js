
const routes = [
  {
    path: '/',
    component: () => import('layouts/MainLayout.vue'),
    redirect: '/b',
    children: [
      { path: '/b/:bottleId', component: () => import('pages/BottlePage.vue') },
      { path: '/b', component: () => import('pages/BottlePage.vue') },
      { path: '/settings', component: () => import('pages/SettingsPage.vue') },
      { path: '/about', component: () => import('pages/AboutPage.vue') },
    ]
  },

  // Always leave this as last one,
  // but you can also remove it
  {
    path: '/:catchAll(.*)*',
    component: () => import('pages/ErrorNotFound.vue')
  }
]

export default routes
