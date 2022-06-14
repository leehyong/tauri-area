import { createApp } from 'vue'
import {Cascader, Input} from "ant-design-vue";
import App from './App.vue'
import 'ant-design-vue/dist/antd.css';
const app = createApp(App);
app.use(Cascader);
app.use(Input);
app.mount('#app')
