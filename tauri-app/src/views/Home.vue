<template>
  <div>
    <el-row :gutter="20">
      <template v-if="panels.length == 0">
        <el-col :span="6" v-for="i in 4" :key="i">
          <el-skeleton style="width: 100%" animated loading>
            <template #template>
              <el-card shadow="hover" class="border-0">
                <template #header>
                  <div class="flex justify-between">
                    <el-skeleton-item variant="text" style="width: 50%"/>
                    <el-skeleton-item variant="text" style="width: 10%"/>
                  </div>
                </template>
                <el-skeleton-item variant="h3" style="width: 80%"/>
                <el-divider/>
                <div class="flex justify-between text-sm text-gray-500">
                  <el-skeleton-item variant="text" style="width: 50%"/>
                  <el-skeleton-item variant="text" style="width: 10%"/>
                </div>
              </el-card>
            </template>
          </el-skeleton>
        </el-col>
      </template>
      <el-col :span="6" :offset="0" v-for="(item, index) in panels" :key="index">
        <el-card shadow="hover" class="border-0">
          <template #header>
            <div class="flex justify-between">
              <span class="text-sm">{{ item.title }}</span>
              <el-tag :type="item.unitColor" effect="plain">
                {{ item.unit }}
              </el-tag>
            </div>
          </template>
          <span class="text-3xl font-bold text-gray-500">
            <CountTo :value="item.value"/>
          </span>
          <el-divider/>
          <div class="flex justify-between text-sm text-gray-500">
            <span>{{ item.subTitle }}</span>
            <span>{{ item.subValue }}</span>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <Index-navs/>

    <el-row :gutter="20">
      <el-col :span="12" offset="0">
        <IndexChart/>
      </el-col>
      <el-col :span="12" offset="0"></el-col>
    </el-row>
  </div>
</template>

<script setup lang="ts">
import CountTo from "@/components/CountTo.vue";
import {useAuthStore} from '@/store/auth'
import {ref} from "vue";
import IndexNavs from "@/components/IndexNavs.vue";
import IndexChart from "@/components/IndexChart.vue";

const authStore = useAuthStore()

const data = [
  {
    title: "支付订单",
    unit: " 年",
    value: 55,
    subTitle: "总支付订单",
    subValue: "55"
  },
  {
    title: "订单量",
    unit: " 周",
    value: 666,
    subTitle: "转化率",
    subValue: "66%"
  },
  {
    title: "销售额",
    unit: " 年",
    value: 3.54,
    subTitle: "总销售额",
    subValue: "8.66"
  },
  {
    title: "新增用户",
    unit: " 年",
    value: 17,
    subTitle: "总用户",
    subValue: 17
  },
]

const panels = ref(data)

const getStatistics = async () => {

}
</script>
<style>

</style>