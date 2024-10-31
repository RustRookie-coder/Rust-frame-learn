<script setup lang="ts">

import * as echarts from 'echarts'
import {ref} from "vue";
const current = ref("week")
const options = [{
  text: "近一个月",
  value: "month",
},{
  text: "近一周",
  value: "week",
},{
  text: "近24小时",
  value: "hour",
}]

const handleChoose = (type) => {
  current.value = type
}
</script>

<template>
  <el-card shadow="never">
    <template #header>
      <div class="flex justify-between">
        <span class="text-sm">订单统计</span>
        <div>
          <el-check-tag v-for="(item, index) in options"
                        :key="index"
                        :checked="current == item.value"
                        style="margin-right: 8px"
          @click="handleChoose(item.value)">
            {{ item.text }}
          </el-check-tag>
        </div>
      </div>
    </template>

    <div id="chart" style="width: 100%; height: 300px">

    </div>
  </el-card>
</template>

<style scoped>

</style>