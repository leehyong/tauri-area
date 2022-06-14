import { createApp } from 'vue'
import {Cascader} from "ant-design-vue";
import App from './App.vue'
import 'ant-design-vue/dist/antd.css';
const app = createApp(App);
app.use(Cascader);
app.mount('#app')
