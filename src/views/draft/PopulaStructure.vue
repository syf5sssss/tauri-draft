<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core';

const lists = ref([]); // 初始化为空数组
const chartData = ref();
const chart2Data = ref();
const chartOptions = ref();
const stacket = ref(false);

onMounted(() => {
  // 初始化代码
})

async function getlist() {
  lists.value = await invoke('popula_list');
  console.log("获取的数据列表:", lists.value);
}

function switchStacket() {
  stacket.value = !stacket.value;
}
async function getchart() {
  // 传递lists.value给setChartData
  chartData.value = setChartData(lists.value);
  chartOptions.value = setChartOptions();
}

async function getchart2() {
  chart2Data.value = setChart2Data(lists.value);
  chartOptions.value = setChartOptions();
}

// 修改函数以接收数据
const setChartData = (dataList) => {
  if (!dataList || dataList.length === 0) {
    console.error("数据列表为空");
    return {
      labels: [],
      datasets: []
    };
  }

  const documentStyle = getComputedStyle(document.documentElement);

  // 过滤1997-2023年的有效数据
  const filteredData = dataList.filter(item =>
    item.year >= 1997 &&
    item.year <= 2023 &&
    item.newborn !== null &&
    item.death !== null
  ).sort((a, b) => a.year - b.year); // 按年份排序

  // 提取年份作为标签
  const labels = filteredData.map(item => item.year.toString());

  // 提取新生儿数据
  const newbornData = filteredData.map(item => item.newborn);

  // 提取死亡数据
  const deathData = filteredData.map(item => item.death);

  //总人口
  const populaData = filteredData.map(item => item.population);

  return {
    labels: labels,
    datasets: [
      {
        type: 'line',
        label: '人口',
        borderColor: documentStyle.getPropertyValue('--p-orange-500'),
        borderWidth: 2,
        fill: false,
        tension: 0.4,
        data: populaData
      },
      {
        type: 'bar',
        label: '新生儿',
        backgroundColor: documentStyle.getPropertyValue('--p-cyan-500'),
        data: newbornData
      },
      {
        type: 'bar',
        label: '死亡者',
        backgroundColor: documentStyle.getPropertyValue('--p-gray-500'),
        data: deathData
      }
    ]
  };
};


// 修改函数以接收数据
const setChart2Data = (dataList) => {
  if (!dataList || dataList.length === 0) {
    console.error("数据列表为空");
    return {
      labels: [],
      datasets: []
    };
  }

  const documentStyle = getComputedStyle(document.documentElement);

  // 过滤1990-2023年的有效数据
  const filteredData = dataList.filter(item =>
    item.year >= 1990 &&
    item.year <= 2023 &&
    item.newborn !== null &&
    item.death !== null
  ).sort((a, b) => a.year - b.year); // 按年份排序

  // 提取年份作为标签
  const labels = filteredData.map(item => item.year.toString());

  // 提取新生儿数据
  const y0y14Data = filteredData.map(item => item.y0_y14);

  // 提取死亡数据
  const y15y64Data = filteredData.map(item => item.y15_y64);

  //
  const over65Data = filteredData.map(item => item.over65);

  //总人口
  const populaData = filteredData.map(item => item.population);

  return {
    labels: labels,
    datasets: [
      {
        type: 'line',
        label: '人口',
        borderColor: documentStyle.getPropertyValue('--p-orange-500'),
        borderWidth: 2,
        fill: false,
        tension: 0.4,
        data: populaData
      },
      {
        label: '少年群体',
        backgroundColor: documentStyle.getPropertyValue('--p-cyan-500'),
        borderColor: documentStyle.getPropertyValue('--p-cyan-500'),
        data: y0y14Data
      },
      {
        label: '青年群体',
        backgroundColor: documentStyle.getPropertyValue('--p-gray-500'),
        borderColor: documentStyle.getPropertyValue('--p-gray-500'),
        data: y15y64Data
      },
      {
        label: '老年群体',
        backgroundColor: documentStyle.getPropertyValue('--p-purple-500'),
        borderColor: documentStyle.getPropertyValue('--p-purple-500'),
        data: over65Data
      }
    ]
  };
};

// 保持原有setChartOptions不变
const setChartOptions = () => {
  const documentStyle = getComputedStyle(document.documentElement);
  const textColor = documentStyle.getPropertyValue('--p-text-color');
  const textColorSecondary = documentStyle.getPropertyValue('--p-text-muted-color');
  const surfaceBorder = documentStyle.getPropertyValue('--p-content-border-color');

  return {
    maintainAspectRatio: false,
    aspectRatio: 0.8,
    plugins: {
      tooltips: {
        mode: 'index',
        intersect: false
      },
      legend: {
        labels: {
          color: textColor
        }
      }
    },
    scales: {
      x: {
        stacked: stacket.value,
        ticks: {
          color: textColorSecondary
        },
        grid: {
          color: surfaceBorder
        }
      },
      y: {
        stacked: stacket.value,
        ticks: {
          color: textColorSecondary
        },
        grid: {
          color: surfaceBorder
        }
      }
    }
  };
}
</script>

<template>
  <Button @click="getlist" class="ml-2 mt-2" fluid>获取数据</Button>
  <Button @click="switchStacket" class="ml-2 mt-2" fluid>堆叠还是平铺</Button>
  <Button @click="getchart" class="ml-2 mt-2" fluid>生成出生死亡图表</Button>
  <Button @click="getchart2" class="ml-2 mt-2" fluid>生成年龄结构图表</Button>

  <div class="card mt-4">
    <Chart type="bar" :data="chartData" :options="chartOptions" class="h-[30rem]" />
  </div>

  <div class="card mt-4">
    <Chart type="bar" :data="chart2Data" :options="chartOptions" class="h-[30rem]" />
  </div>
</template>