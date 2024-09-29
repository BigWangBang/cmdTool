// router/index.js
import { createRouter, createWebHistory } from 'vue-router';
import LogView from '../views/LogView.vue';
import Command from '../views/CommandList.vue';

const routes = [
  // {
  //   path: '/',
  //   name: 'LogView',
  //   component: LogView
  // },
  {
    path: '/',
    name: 'Command',
    component: Command
  }
];

const router = createRouter({
  history: createWebHistory(),
  routes
});

export default router;