<script setup lang="ts">
import { h, ref, reactive } from 'vue'
import { invoke } from "@tauri-apps/api/tauri";
import { Record } from "@/modules/Record";
import { MdFlash, MdFlashOff, MdRemoveCircle, MdCreate, MdAddCircle } from '@vicons/ionicons4';
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
    useMessage,
    NIcon,
    NButtonGroup,
    NPopover
} from 'naive-ui';

const message = useMessage();

// 记录项表单
const formRef = ref<FormInst | null>(null)

// 表格配置
const columns = [
    {
        title: '远程地址',
        key: 'remote_host',
        width: 200,
        ellipsis: true,
        render(row: Record) {
            return h(
                NPopover,
                {
                    trigger: 'hover',
                    placement: 'bottom'
                },
                {
                    trigger: () => h(
                        NGradientText,
                        {
                            style: {
                                marginRight: '6px',
                                fontSize: '13px',
                                cursor: 'pointer'
                            },
                            type: 'info',
                        },
                        {
                            default: () => row.remote_host
                        }
                    ),
                    default: () => h(
                        'span',
                        {},
                        {
                            default: () => row.remote_host
                        }
                    )
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
                    type: 'error',
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
                        type: 'success',
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
        width: 130,
        align: 'center',
        render(row: Record, rowIndex: number) {
            console.log(row.pid);
            return h(
                NButtonGroup,
                {},
                () => [
                    h(
                        NButton,
                        {
                            strong: true,
                            type: row.pid > 0 ? "warning" : "primary",
                            size: "small",
                            loading: btnLoading.value,
                            onClick: () => {
                                if (row.pid > 0) {
                                    console.log("关闭");
                                    closePort(rowIndex)
                                } else if (row.pid <= 0) {
                                    console.log("开启");
                                    openPort(rowIndex)
                                }
                            }
                        },
                        {
                            icon: () => h(NIcon, null, {
                                default: () => {
                                    if (row.pid > 0) {
                                        return h(MdFlashOff)
                                    } else {
                                        return h(MdFlash)
                                    }
                                }
                            }),
                            default: () => h("span", row.pid > 0 ? "关闭" : "开启")
                        }
                    ),
                    h(
                        NButton,
                        {
                            strong: true,
                            type: "info",
                            disabled: row.pid > 0 ? true : false,
                            size: "small",
                            style: {
                                marginLeft: '8px'
                            },
                            onClick: () => openEditForm(rowIndex)
                        },
                        {
                            icon: () => h(NIcon, null, () => h(MdCreate, null, {})),
                            // default: () => "编辑"
                        }
                    ),
                    h(
                        NButton,
                        {
                            strong: true,
                            disabled: row.pid > 0 ? true : false,
                            type: "error",
                            size: "small",
                            round: true,
                            style: {
                                marginLeft: '8px'
                            },
                            onClick: () => openDel(rowIndex)
                        },
                        {
                            icon: () => h(NIcon, null, () => h(MdRemoveCircle, null, {})),
                            // default: () => "删除"
                        }
                    )
                ]
            )
        }
    }
];

const btnLoading = ref(false);

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

// 记录列表
const recordData = reactive<RecordData>({
    item: <Record>{},
    list: []
});

// 获取记录列表
function getRecords() {
    invoke("get_records").then((data) => {
        if (typeof data === 'string') {
            recordData.list = JSON.parse(data);
            console.log(recordData.list);
        } else {
            console.error('Data is not a string:', data);
        }
    })
        .catch((err) => {
            console.error(err);
            // 处理错误情况
            message.error(err);
        });
}
// 加载初始数据
getRecords();


// 添加记录窗口
const showModal = ref(false);
// 添加编辑控制
const editIndex = ref(-1);


// 创建记录
function createRecord() {
    let saveData = JSON.parse(JSON.stringify(recordData.list));
    if (editIndex.value > -1) {
        saveData[editIndex.value] = recordData.item;
    } else {
        saveData.push(recordData.item);
    }

    saveRecord(saveData).then((status) => {
        if (status) {
            message.success('操作成功');
        } else {
            message.error('操作失败');
        }
    }).catch((err) => {
        message.error(err);
    });
    showModal.value = false;
    editIndex.value = -1;
}

// 编辑记录窗口
function openEditForm(index: number) {
    recordData.item = { ...recordData.list[index] };
    editIndex.value = index;
    showModal.value = true;
}

// 添加记录窗口
function openCreateForm() {
    recordData.item = {
        local_host: '0.0.0.0',
        local_port: 8080,
        remote_host: '',
        remote_port: 80,
        protocol: ['tcp', 'udp'],
        pid: 0
    };
    editIndex.value = -1;
    showModal.value = true;
}


// 保存记录
async function saveRecord(saveData: Array<Record>) {
    return await invoke("save_record", { jsonStr: JSON.stringify(saveData) }).then((records) => {
        if (typeof records === 'string') {
            recordData.list = JSON.parse(records);
            return true;
        } else {
            console.error('数据格式错误:', records);
            // 处理错误情况
            return false;
        }
    }).catch((err) => {
        console.error(err);
        message.error(err);
        // 处理错误情况
        return false;
    });
}

// 打开端口转发
function openPort(index: number) {
    console.log('open port');
    btnLoading.value = true;
    const item = recordData.list[index];
    invoke("open_port", { jsonStr: JSON.stringify(item) }).then((pid) => {
        const id: number = pid as number;
        if (id > 0) {
            recordData.list[index].pid = id;
            saveRecord(recordData.list);
            message.success('开启成功');
        } else {
            message.error('开启失败');
        }
        btnLoading.value = false;

    })
        .catch((err) => {
            console.error(err);
            // 处理错误情况
            message.error(err);
            btnLoading.value = false;

        });

}

// 关闭端口
function closePort(index: number) {
    btnLoading.value = true;
    invoke("close_port", { pid: recordData.list[index].pid }).then((pid) => {
        if (pid) {
            recordData.list[index].pid = 0;
            saveRecord(recordData.list);
            message.success('关闭成功');
        } else {
            message.error('关闭失败');
        }
        btnLoading.value = false;
    })
        .catch((err) => {
            console.error(err);
            // 处理错误情况
            message.error(err);
            btnLoading.value = false;
        });
}

// 删除确认框
const delModal = ref(false);

// 删除的记录下标
const delIndex = ref<number>(-1);

// 确认删除
const submitDel = () => {
    delModal.value = false;

    if (delIndex.value < 0) {
        message.error("未找到删除项");
        return false;
    }

    let saveData = JSON.parse(JSON.stringify(recordData.list));
    const item = saveData[delIndex.value];

    if (item.status > 0) {
        message.warning("先关闭转发后再删除");
        return false;
    }

    saveData.splice(delIndex.value, 1)
    saveRecord(saveData).then((status) => {
        if (status) {
            message.success('删除成功');
        } else {
            message.error('删除失败');
        }
    }).catch((err) => {
        message.error(err);
    });
};

// 取消删除
const cancelDel = () => {
    delModal.value = false;
    delIndex.value = -1;
};

// 打开删除框
function openDel(id: number) {
    delModal.value = true;
    delIndex.value = id;
};

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
            createRecord();
        } else {
            console.log(recordData.item)
            console.log(errors[0][0].message)
            message.error(<string>errors[0][0].message)
        }
    })
};

</script>


<template>
    <div class="container">
        <n-data-table style="height:100%;" size="small" :columns="columns" :data="recordData.list"
            :pagination="pagination" :bordered="false" :single-line="false" />
        <n-button class="add-btn" type="info" size="medium" @click="openCreateForm">
            <template #icon>
                <n-icon>
                    <MdAddCircle />
                </n-icon>
            </template>
        </n-button>
    </div>

    <!-- 添加记录 -->
    <n-modal v-model:show="showModal" class="custom-card" preset="card" :style="bodyStyle"
        :title="editIndex > -1 ? '编辑记录' : '添加记录'" size="small" :bordered="false" :segmented="segmented"
        :mask-closable="false" footer-style="display: flex;flex-direction: row-reverse;">
        <template #header-extra>
        </template>
        <n-form ref="formRef" :model="recordData.item" :rules="rules">
            <n-form-item path="local_host" label="本地地址">
                <n-input v-model:value="recordData.item.local_host" :disabled="true" placeholder="输入本地IP"
                    @keydown.enter.prevent />
            </n-form-item>
            <n-form-item path="local_port" label="本地端口">
                <n-input-number v-model:value="recordData.item.local_port" placeholder="输入本地端口"
                    @keydown.enter.prevent />
            </n-form-item>
            <n-form-item path="remote_host" label="远程地址（IP、域名）">
                <n-input v-model:value="recordData.item.remote_host" placeholder="输入远程地址（IP或域名）"
                    @keydown.enter.prevent />
            </n-form-item>
            <n-form-item path="remote_port" label="远程端口">
                <n-input-number v-model:value="recordData.item.remote_port" placeholder="输入远程端口"
                    @keydown.enter.prevent />
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

    <n-modal v-model:show="delModal" preset="dialog" title="确认" content="确定要删除？" type="warning" :mask-closable="false"
        positive-text="确认" negative-text="取消" @positive-click="submitDel" @negative-click="cancelDel" />
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