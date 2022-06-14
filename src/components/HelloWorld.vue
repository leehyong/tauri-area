<template>
  <!--  <a-cascader-->
  <!--      v-model:value="value"-->
  <!--      :options="options"-->
  <!--      :field-names="{ label: 'name', value: 'code'}"-->
  <!--      :load-data="loadData"-->
  <!--      placeholder="Please select"-->
  <!--      :show-search="{ filter }"-->
  <!--      change-on-select-->
  <!--  />-->
  <div>
    <h1>地址选择器 </h1>
  </div>
  <div>
    <a-cascader
        v-model:value="value"
        :options="options"
        size="large"
        style="width: 80%"
        :field-names="{ label: 'name', value: 'code'}"
        :loadData="loadData"
        placeholder="Please select"
        change-on-select
    />
  </div>
  <div style="margin-top: 10px">
    <h3>已选择的地址:<span style="color: royalblue;margin-left: 20px">{{ addr }}</span></h3>
  </div>

</template>
<script>
// With the Tauri API npm package:
import {invoke} from '@tauri-apps/api/tauri'
// With the Tauri global script, enabled when `tauri.conf.json > build > withGlobalTauri` is set to true:

const CITY = 2;
const AREA = 3;
const STREET = 4;
const VILLAGE = 5;

export default {
  name: "HelloWord",
  data() {
    return {
      options: null,
      value: [],
    }
  },
  created() {
    let ths = this;
    invoke("all_provinces")
        .then((res) => {
          ths.options = (res || []);
          ths.options.forEach(item => item.isLeaf = false);
          console.log(ths.options)

        }).catch(err => {
      console.error(err)
    })
  },
  methods: {
    filter(inputValue, path) {
      return path.some(option => option.name.indexOf(inputValue) > -1);
    },

    loadData(selectedOptions) {
      const typ = (selectedOptions.length || 0) + 1;
      let method = "";
      const targetOption = selectedOptions[selectedOptions.length - 1];
      // console.log('selectedOptions', selectedOptions, targetOption)
      const parantCode = targetOption.code;
      switch (typ) {
        case CITY:
          method = "get_city";
          break;
        case AREA:
          method = "get_area";
          break
        case STREET:
          method = "get_street";
          break
        case VILLAGE:
          method = "get_village";
          break
        default:
          return;
      }
      targetOption.loading = true;
      invoke(method, {code: parantCode})
          .then(res => {
            targetOption.loading = false;
            targetOption.children = res || [];
            if (typ === VILLAGE) {
              targetOption.children.forEach(item => item.isLeaf = true);
            } else {
              targetOption.children.forEach(item => item.isLeaf = false);
            }
            console.log('targetOption.children', targetOption.children)
          })
          .catch(err => {
            targetOption.loading = false;
            console.error(err)
          })
    }
  },
  computed: {
    addr() {
      if (this.value.length === 0) return "";
      let s = [];
      let curOptions = this.options;
      let curOption;
      let idx;
      for (let code of this.value) {
        idx = curOptions.findIndex(item => item.code == code)
        if (idx == -1) break
        curOption = curOptions [idx];
        if (curOption.name !== "市辖区")
          s.push(curOption.name);
        curOptions = curOption.children || [];
      }
      return s.join("");
    },
  }

}

</script>
<style scoped>
a {
  color: #42b983;
}
</style>
