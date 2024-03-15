<script setup lang="ts">
import { h, ref, reactive  } from 'vue'
import { invoke } from "@tauri-apps/api/tauri";
import {
    NButton,
    NDataTable,
    NTag,
    NGradientText,
    NModal,
    NForm,
    NInput,
    NFormItem,
    NCheckbox,
    NCheckboxGroup,
    NSpace,
    FormRules
} from 'naive-ui';

// 配置项表单
const formRef = ref<FormInst | null>(null)

// 表格配置
const columns = [
    {
        title: '本地地址',
        key: 'local_host',
        width: 200,
        // resizable: true,
        ellipsis: true,
        render(row) {
            return h(
                NGradientText,
                {
                    style: {
                        marginRight: '6px',
                        fontSize: '13px',
                    },
                    type: 'success',
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
        title: '远程地址',
        key: 'remote_host',
        
        width: 200,
        ellipsis: true,
        render(row) {
            return h(
                NGradientText,
                {
                    style: {
                        marginRight: '6px',
                        fontSize: '13px',
                    },
                    type: 'info',
                },
                {
                    default: () => row.local_host
                }
            )
        }
    },
    {
        title: '远程端口',
        key: 'remote_port',
        
        align: 'center',
        width: 60,
        render(row) {
            return h(
                NTag,
                {
                    style: {
                        marginRight: '6px',
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
        key: 'status',
        width: 100,
        align: 'center',
        render(row) {
            return [
                h(
                    NButton,
                    {
                        strong: true,
                        // tertiary: true,
                        type: "primary",
                        size: "small",
                        onClick: () => play(row)
                    },
                    { default: () => "开启" }
                ),
                // h(
                //   NButton,
                //   {
                //     strong: true,
                //     // tertiary: true,
                //     type: "warning",
                //     size: "small",
                //     onClick: () => play(row)
                //   },
                //   { default: () => "关闭" }
                // ),
                h(
                    NButton,
                    {
                        strong: true,
                        type: "error",
                        size: "small",
                        style: {
                            marginLeft: '8px'
                        },
                        onClick: () => play(row)
                    },
                    { default: () => "删除" }
                )
            ]
        }
    }
];

// 配置列表
const configs = reactive([
    {
        local_host: '127.0.0.1',
        local_port: '80800',
        remote_host: '2132:0568:0123:1223:0DA8:0D45:0000:52D3',
        remote_port: '8080',
        status: '删除'
    },
    {
        local_host: '2132:0568:0123:1223:0DA8:0D45:0000:52D3',
        local_port: '8080',
        remote_host: '0.0.0.0',
        remote_port: '8080',
        status: '删除'
    },
    {
        local_host: '[::]',
        local_port: '8080',
        remote_host: '0.0.0.0',
        remote_port: '8080',
        status: '删除'
    },
]);

// 页码控件
const pagination = {
    pageSize: 2,
};

const bodyStyle = {
    width: '400px'
};
const segmented = {
    content: 'soft',
    footer: 'soft'
};

// 配置项
const configItem = ref({
    local_host: '0.0.0.0',
    local_port: '80800',
    remote_host: '2132:0568:0123:1223:0DA8:0D45:0000:52D3',
    remote_port: '8080',
    protocol: ['udp', 'tcp']
});

// 添加配置
async function addConfig() {
    console.log(configItem.value);
    const newConfig = await invoke("add_config", { data: JSON.stringify(configItem.value) });
    configItem.value = JSON.parse(newConfig);
    configs.push(configItem.value);
    console.log(configItem.value);
}

// 添加配置窗口
const showModal = ref(false);

// 表单验证规则
const rules: FormRules = {
    local_host: {
        required: true,
        message: '请输入本地地址',
        trigger: 'blur'
    },
    local_port: {
        required: true,
        message: '请输入本地端口',
        trigger: 'blur'
    },
    remote_host: {
        required: true,
        message: '请输入远程地址',
        trigger: 'blur'
    },
    remote_port: {
        required: true,
        message: '请输入远程端口',
        trigger: 'blur'
    },
    protocol: {
        type: 'array',
        required: true,
        message: '请选择协议',
        trigger: 'change'
    }
}

// 验证表单
const handleValidateButtonClick = (e: MouseEvent) => {
    e.preventDefault()
    formRef.value?.validate((errors) => {
        if (!errors) {
            addConfig();
            window.$message.success('验证成功')
        } else {
            console.log(errors)
            window.$message.error('验证失败')
        }
    })
};

</script>


<template>
    <div class="container">
        <n-data-table style="height:100%;" size="small" :columns="columns" :data="configs" :pagination="pagination"
            :bordered="false" :single-line="false" />
        <n-button class="add-btn" type="info" size="large" @click="showModal = true">添加</n-button>
    </div>

    <!-- 添加转发 -->
    <n-modal v-model:show="showModal" class="custom-card" preset="card" :style="bodyStyle" title="添加转发" size="small"
        :bordered="false" :segmented="segmented" footer-style="display: flex;flex-direction: row-reverse;">
        <template #header-extra>
        </template>
        <n-form ref="formRef" :model="configItem" :rules="rules">
            <n-form-item path="local_host" label="本地地址（IP、域名）">
                <n-input v-model:value="configItem.local_host" @keydown.enter.prevent />
            </n-form-item>
            <n-form-item path="local_port" label="本地端口">
                <n-input v-model:value="configItem.local_port" @keydown.enter.prevent />
            </n-form-item>
            <n-form-item path="remote_host" label="远程地址（IP、域名）">
                <n-input v-model:value="configItem.remote_host" @keydown.enter.prevent />
            </n-form-item>
            <n-form-item path="remote_port" label="远程端口">
                <n-input v-model:value="configItem.remote_port" @keydown.enter.prevent />
            </n-form-item>
            <n-form-item label="转发协议" path="protocol">
                <n-checkbox-group v-model:value="configItem.protocol">
                    <n-space>
                        <n-checkbox value="tcp">
                            TCP
                        </n-checkbox>
                        <n-checkbox value="udp">
                            UDP
                        </n-checkbox>
                    </n-space>
                </n-checkbox-group>
            </n-form-item>
        </n-form>
        <template #footer>
            <n-button class="add-btn" type="info" size="large" @click="handleValidateButtonClick">确认</n-button>
        </template>
    </n-modal>
</template>

<style lang="scss" scoped>
.container {
    height: 100%;

    :deep(.n-data-table .n-data-table__pagination) {
        margin-top: 0;
        margin-right: 15px;
        height: 35px;
    }

    .add-btn {
        position: fixed;
        left: 15px;
        bottom: 10px;
    }

    .footer-style {
        display: flex;
        flex-direction: row-reverse;
    }
}
</style>

<style></style>