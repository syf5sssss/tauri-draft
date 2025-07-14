import AppLayout from '@/layout/AppLayout.vue';
import { createRouter, createWebHistory } from 'vue-router';

const router = createRouter({
    history: createWebHistory(),
    routes: [
        {
            path: '/',
            component: AppLayout,
            children: [
                {
                    path: '/',
                    name: 'dashboard',
                    component: () => import('@/views/Dashboard.vue')
                },
                {
                    path: '/draft/print',
                    name: 'print',
                    component: () => import('@/views/draft/PrintLayout.vue')
                },
                {
                    path: '/draft/os',
                    name: 'os',
                    component: () => import('@/views/draft/OS.vue')
                },
                {
                    path: '/draft/config',
                    name: 'config',
                    component: () => import('@/views/draft/Config.vue')
                },
                {
                    path: '/draft/thread',
                    name: 'thread',
                    component: () => import('@/views/draft/Thread.vue')
                },
                {
                    path: '/draft/hashmap',
                    name: 'hashmap',
                    component: () => import('@/views/draft/HashMap.vue')
                },
                {
                    path: '/draft/vec',
                    name: 'vec',
                    component: () => import('@/views/draft/Vec.vue')
                },
                {
                    path: '/draft/structure',
                    name: 'structure',
                    component: () => import('@/views/draft/PopulaStructure.vue')
                },
                {
                    path: '/draft/blazeface',
                    name: 'blazeface',
                    component: () => import('@/views/draft/BlazeFace.vue')
                },
                {
                    path: '/draft/socket/tcpserver',
                    name: 'tcpserver',
                    component: () => import('@/views/draft/TcpServer.vue')
                },
                {
                    path: '/draft/socket/tcpclient',
                    name: 'tcpclient',
                    component: () => import('@/views/draft/TcpClient.vue')
                },
                {
                    path: '/draft/socket/udpbroadcaster',
                    name: 'udpbroadcaster',
                    component: () => import('@/views/draft/UdpBroadCaster.vue')
                },
                {
                    path: '/draft/socket/udpmulticast',
                    name: 'udpmulticast',
                    component: () => import('@/views/draft/UdpMultiCast.vue')
                },
                {
                    path: '/draft/fileio',
                    name: 'fileio',
                    component: () => import('@/views/draft/File.vue')
                },
                {
                    path: '/draft/crud',
                    name: 'crud-form',
                    component: () => import('@/views/draft/CRUD.vue')
                },
                {
                    path: '/uikit/formlayout',
                    name: 'formlayout',
                    component: () => import('@/views/uikit/FormLayout.vue')
                },
                {
                    path: '/uikit/input',
                    name: 'input',
                    component: () => import('@/views/uikit/InputDoc.vue')
                },
                {
                    path: '/uikit/button',
                    name: 'button',
                    component: () => import('@/views/uikit/ButtonDoc.vue')
                },
                {
                    path: '/uikit/table',
                    name: 'table',
                    component: () => import('@/views/uikit/TableDoc.vue')
                },
                {
                    path: '/uikit/list',
                    name: 'list',
                    component: () => import('@/views/uikit/ListDoc.vue')
                },
                {
                    path: '/uikit/tree',
                    name: 'tree',
                    component: () => import('@/views/uikit/TreeDoc.vue')
                },
                {
                    path: '/uikit/panel',
                    name: 'panel',
                    component: () => import('@/views/uikit/PanelsDoc.vue')
                },

                {
                    path: '/uikit/overlay',
                    name: 'overlay',
                    component: () => import('@/views/uikit/OverlayDoc.vue')
                },
                {
                    path: '/uikit/media',
                    name: 'media',
                    component: () => import('@/views/uikit/MediaDoc.vue')
                },
                {
                    path: '/uikit/message',
                    name: 'message',
                    component: () => import('@/views/uikit/MessagesDoc.vue')
                },
                {
                    path: '/uikit/file',
                    name: 'file',
                    component: () => import('@/views/uikit/FileDoc.vue')
                },
                {
                    path: '/uikit/menu',
                    name: 'menu',
                    component: () => import('@/views/uikit/MenuDoc.vue')
                },
                {
                    path: '/uikit/charts',
                    name: 'charts',
                    component: () => import('@/views/uikit/ChartDoc.vue')
                },
                {
                    path: '/uikit/misc',
                    name: 'misc',
                    component: () => import('@/views/uikit/MiscDoc.vue')
                },
                {
                    path: '/uikit/timeline',
                    name: 'timeline',
                    component: () => import('@/views/uikit/TimelineDoc.vue')
                },
                {
                    path: '/pages/empty',
                    name: 'empty',
                    component: () => import('@/views/pages/Empty.vue')
                },
                {
                    path: '/pages/crud',
                    name: 'crud',
                    component: () => import('@/views/pages/Crud.vue')
                },
                {
                    path: '/documentation',
                    name: 'documentation',
                    component: () => import('@/views/pages/Documentation.vue')
                }
            ]
        },
        {
            path: '/landing',
            name: 'landing',
            component: () => import('@/views/pages/Landing.vue')
        },
        {
            path: '/pages/notfound',
            name: 'notfound',
            component: () => import('@/views/pages/NotFound.vue')
        },

        {
            path: '/auth/login',
            name: 'login',
            component: () => import('@/views/pages/auth/Login.vue')
        },
        {
            path: '/auth/access',
            name: 'accessDenied',
            component: () => import('@/views/pages/auth/Access.vue')
        },
        {
            path: '/auth/error',
            name: 'error',
            component: () => import('@/views/pages/auth/Error.vue')
        }
    ]
});

export default router;
