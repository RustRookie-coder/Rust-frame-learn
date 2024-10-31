<script setup lang="ts">

import * as echarts from 'echarts'
import {onMounted, ref, onUnmounted} from "vue";
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


const periodData = ref()
const realData = ref()

const handleChoose = (type) => {
  current.value = type
  switch (current.value) {
    case "month": {
      periodData.value = ["Jan", "Feb", "Mar", "Api", "May", "June", "July", "Aug", "Sep", "Oct", "Nov", "Dec"]
      realData.value = [110, 200, 350, 440, 110, 509, 89, 121, 333, 21, 66, 99]
      initChart();
      break;
    }
    case "week": {
      periodData.value = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun']
      realData.value = [120, 209, 150, 80, 70, 110, 120]
      initChart();
      break;
    }
    case "hour" : {
      periodData.value = ["12:00", "1:00" ,"2:00" , "3:00" ,"4:00" ,"5:00" ,"6:00", "7:00", "8:00", "9:00" ,"10:00", "11:00"]
      realData.value = [110, 200, 350, 440, 110, 509, 89, 121, 333, 21, 66, 99]
      initChart();
      break;
    }
  }
  console.log("type:" + current.value)
  console.log("periodData:" + periodData.value)
  console.log("realData:" + realData.value)
}
const chartRef = ref<HTMLDivElement | null>(null)
let chartInstance: echarts.ECharts | null = null;

const initChart = () => {
  if(chartRef.value) {
    chartInstance = echarts.init(chartRef.value); // Initialize ECharts
    chartInstance.setOption({
      xAxis: {
        type: 'category',
        data: periodData.value
      },

      yAxis: {
        type: "value"
      },
      series: [
        {
          data: realData.value,
          type: 'bar',
          showBackground: true,
          backgroundStyle: {
            color: 'rgba(180, 180, 180, 0.2)'
          }
        }
      ]
    })
  }
}

// Handle window resize to adjust the chart size
const resizeChart = () => {
  chartInstance?.resize();
}

onMounted(() => {
  handleChoose(current.value)
  initChart(); // Initialize the chart when the component is mounted
  window.addEventListener('resize', resizeChart); // Add resize event listener
})

onUnmounted(() => {
  window.removeEventListener('resize', resizeChart); // Clean up event listener
  chartInstance?.dispose(); // Dispose of the chart instance when unmounted
});

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

    <div ref="chartRef" style="width: 100%; height: 300px">

    </div>
  </el-card>
</template>

<style scoped>

</style>