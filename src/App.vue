<script setup lang="ts">
import { h } from 'vue'
import { NButton, NDataTable, NTag } from 'naive-ui';

const columns = [
  {
    title: '本地IP',
    key: 'local_host',
    width: 200,
    // resizable: true,
    render(row) {
      return h(
        NTag,
        {
          style: {
            marginRight: '6px'
          },
          type: 'success',
          bordered: false,
          size: 'small'
        },
        {
          default: () => row.local_host
        }
      )
    }
  },
  {
    title: '本地端口',
    key: 'local_port',
    // resizable: true,
    align: 'center',
    width: 60,
    render(row) {
      return h(
        NTag,
        {
          style: {
            marginRight: '6px'
          },
          type: 'success',
          bordered: false,
          size: 'small'
        },
        {
          default: () => row.local_port
        }
      )
    }
  },
  {
    title: '远程IP',
    key: 'remote_host',
    // resizable: true,
    width: 200,
    render(row) {
      return h(
        NTag,
        {
          style: {
            marginRight: '6px'
          },
          type: 'info',
          bordered: false,
          size: 'small'
        },
        {
          default: () => row.remote_host
        }
      )
    }
  },
  {
    title: '远程端口',
    key: 'remote_port',
    // resizable: true,
    align: 'center',
    width: 60,
    render(row) {
      return h(
        NTag,
        {
          style: {
            marginRight: '6px'
          },
          type: 'info',
          bordered: false,
          size: 'small'
        },
        {
          default: () => row.remote_port
        }
      )
    }
  },
  {
    title: '操作',
    key: 'action',
    width: 50,
    render(row) {
      return h(
        NButton,
        {
          strong: true,
          tertiary: true,
          size: "small",
          onClick: () => play(row)
        },
        { default: () => "删除" }
      );
    }
  }
];

const data = [
  {
    local_host: '127.0.0.1',
    local_port: '80800',
    remote_host: '2132:0568:0123:1223:0DA8:0D45:0000:52D3',
    remote_port: '8080',
    action: '删除'
  },
  {
    local_host: '2132:0568:0123:1223:0DA8:0D45:0000:52D3',
    local_port: '8080',
    remote_host: '0.0.0.0',
    remote_port: '8080',
    action: '删除'
  },
  {
    local_host: '[::]',
    local_port: '8080',
    remote_host: '0.0.0.0',
    remote_port: '8080',
    action: '删除'
  },
];

const pagination = {
  pageSize: 2,
};
</script>

<template>
  <div class="container">
    <n-data-table :size="small" :min-height="540" :columns="columns" :data="data" :pagination="pagination" :bordered="false" :single-line="false"/>
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
