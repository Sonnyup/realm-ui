<script setup lang="ts">
import { h, ref, reactive } from 'vue'
import { invoke } from "@tauri-apps/api/tauri";
import { Record } from "@/modules/Record";
import {
    NButton,
    NDataTable,
    NTag,
    NGradientText,
    NModal,
    NForm,
    NInput,
    NInputNumber,
    NFormItem,
    NCheckbox,
    NCheckboxGroup,
    NSpace,
    FormRules,
    FormInst,
    useMessage
} from 'naive-ui';

const message = useMessage();

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
        render(row: Record) {
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
        render(row: Record) {
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
        render(row: Record) {
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
        render(row: Record) {
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
        title: '转发协议',
        key: 'protocol',

        align: 'center',
        width: 80,
        render(row: Record) {
            const tags = row.protocol.map((tagKey) => {
                return h(
                    NTag,
                    {
                        style: {
                            marginRight: '6px',
                        },
                        type: 'error',
                        bordered: false,
                        size: 'small'
                    },
                    {
                        default: () => tagKey
                    }
                )
            });
            return tags;
        }
    },
    {
        title: '操作',
        key: 'status',
        width: 100,
        align: 'center',
        render(row: Record) {
            console.log(row)
            return [
                h(
                    NButton,
                    {
                        strong: true,
                        // tertiary: true,
                        type: "primary",
                        size: "small",
                        onClick: () => { }
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
                        onClick: () => { }
                    },
                    { default: () => "删除" }
                )
            ]
        }
    }
];

// 页码控件
const pagination = {
    pageSize: 11,
};

const bodyStyle = {
    width: '400px'
};
const segmented = {
    content: 'soft',
    footer: 'soft'
};

// 数据
type RecordData = {
    item: Record,
    list: Record[];
}

// 配置列表
const recordData = reactive<RecordData>({
    item: {
        local_host: '127.0.0.1',
        local_port: 1080,
        remote_host: '127.0.0.1',
        remote_port: 80,
        protocol: ['tcp', 'udp'],
        status: false
    },
    list: []
});

// 获取配置列表
function getRecords() {
    invoke("get_records").then((data) => {
        if (typeof data === 'string') {
            recordData.list = JSON.parse(data);
            console.log(recordData.list);
        } else {
            console.error('Data is not a string:', data);
            // 处理错误情况  
        }
    })
    .catch((err) => {
        console.error(err);
    });
}
getRecords();

// 新增配置
function insertRecord() {
    console.log(recordData.item);
    invoke("insert_record", { data: JSON.stringify(recordData.item) }).then((records) => {
        if (typeof records === 'string') {
            recordData.list = JSON.parse(records);
            console.log(recordData.item);
        } else {
            console.error('Data is not a string:', records);
            // 处理错误情况  
        }
    });
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
        type: 'number',
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
        type: 'number',
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
            insertRecord();
            message.success('验证成功')
        } else {

            console.log(recordData.item)
            console.log(errors)
            message.error('验证失败')
        }
    })
};

</script>


<template>
    <div class="container">
        <n-data-table style="height:100%;" size="small" :columns="columns" :data="recordData.list"
            :pagination="pagination" :bordered="false" :single-line="false" />
        <n-button class="add-btn" type="info" size="large" @click="showModal = true">添加</n-button>
    </div>

    <!-- 添加转发 -->
    <n-modal v-model:show="showModal" class="custom-card" preset="card" :style="bodyStyle" title="添加转发" size="small"
        :bordered="false" :segmented="segmented" footer-style="display: flex;flex-direction: row-reverse;">
        <template #header-extra>
        </template>
        <n-form ref="formRef" :model="recordData.item" :rules="rules">
            <n-form-item path="local_host" label="本地地址（IP、域名）">
                <n-input v-model:value="recordData.item.local_host" @keydown.enter.prevent />
            </n-form-item>
            <n-form-item path="local_port" label="本地端口">
                <n-input-number v-model:value="recordData.item.local_port" @keydown.enter.prevent />
            </n-form-item>
            <n-form-item path="remote_host" label="远程地址（IP、域名）">
                <n-input v-model:value="recordData.item.remote_host" @keydown.enter.prevent />
            </n-form-item>
            <n-form-item path="remote_port" label="远程端口">
                <n-input-number v-model:value="recordData.item.remote_port" @keydown.enter.prevent />
            </n-form-item>
            <n-form-item label="转发协议" path="protocol">
                <n-checkbox-group v-model:value="recordData.item.protocol">
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