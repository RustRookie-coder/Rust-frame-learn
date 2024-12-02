import { computed, reactive, ref } from "vue";
import {deleteSkus} from "@/api/skus";
import {ElNotification} from "element-plus";

export const useInitTable = (opt = {}) => {
    let searchForm = null;
    let resetSearchForm = null;
    //@ts-ignore
    if(opt.searchForm) {
        //@ts-ignore
        searchForm = reactive({ ...opt.searchForm })
        //@ts-ignore
        resetSearchForm = () => {
            console.log("is success" + JSON.stringify(searchForm))
            //@ts-ignore
            for(const key in opt.searchForm) {
                //@ts-ignore
                searchForm[key] = opt.searchForm[key]
            }
            getData()
        }
    }
    const loading = ref(false)
    const tableData = ref([])
    const total = ref(1)
    const currentPage = ref(1)
    const limit = ref(10)
    const getData = async (p = null) => {
        if(typeof p == "number") {
            currentPage.value = p
        }
        loading.value = true
        //@ts-ignore
        await opt.getList(searchForm).then((res) => {
            //@ts-ignore
            if (opt.onGetListSuccess && typeof opt.onGetListSuccess == "function") {
                //@ts-ignore
                opt.onGetListSuccess(res)
            } else {
                tableData.value = res
            }
        }).finally(() => {
            loading.value = false
        })
    }

    getData()

    // delete notification by backend
    const handleDelete = (id) => {
        //@ts-ignore
        opt.delete(id).then(res => {
            console.log("delete" + res)
        })
    }
    const handleStatusChange = (status, row) => {
        row.statusLoading = true
        console.log(status)
        //@ts-ignore
        opt.updateStatus(row.id, status).then(res => {

        })
    }

    const errorHandler = () => {

    }

    const multipleTableRef = ref(null)
    const multiSelectionIds = ref([])
    const handleSelectionChange = (e) => {
        multiSelectionIds.value = e.map(o => o.id)
        console.log("skus ids:" + multiSelectionIds.value)
    }

    const handleMultiDelete = () => {
        loading.value = true
        //@ts-ignore
        opt.delete(multiSelectionIds.value).then(res => {
            //清空选中
            if(res) {
                ElNotification({
                    message: 'This is a success message',
                    type: 'success',
                })
                if(multipleTableRef.value) {
                    //@ts-ignore
                    multipleTableRef.value.clearSelection()
                }
                getData()
            }
        }).finally(() => loading.value = false)
    }

    return {
        searchForm,
        tableData,
        total,
        currentPage,
        limit,
        loading,
        resetSearchForm,
        getData,
        handleStatusChange,
        handleDelete,
        errorHandler,
        multipleTableRef,
        handleSelectionChange,
        handleMultiDelete,
    }
}
export const useInitForm = (opt = {}) => {
    const drawerRef = ref(null)
    //@ts-ignore
    const defaultForm = opt.form
    const formRef = ref(null)
    const form = reactive({})
    const editId = ref(0)
    const drawerTitle = computed(() => editId.value ? "修改" : "新增")
    //@ts-ignore
    const rules = opt.rules || {}
    const resetForm = (row = false) => {
        //@ts-ignore
        if(formRef.value) formRef.value.clearValidate()
        for(const key in defaultForm) {
            form[key] = row[key]
        }
    }
    const handleSubmit = () => {
        //@ts-ignore
        formRef.value.validate((valid) => {
            if (!valid) return
            //@ts-ignore
            drawerRef.value.showLoading()
            let body = {}
            //@ts-ignore
            if(opt.beforeSubmit && opt.beforeSubmit == "function") {
                //@ts-ignore
                body = opt.beforeSubmit({ ...form })
            } else {
                body = form
            }
            //@ts-ignore
            const fun = editId.value ? opt.update(editId.value, body) : opt.create(body)
            fun.then((res) => {
                //todo
                console.log("update or create" + res)
            })
            //@ts-ignore
            drawerRef.value.hideLoading()
            //@ts-ignore
            drawerRef.value.close()
        })
    }

    const handleEdit = (row) => {
        editId.value = row.id
        resetForm(row)
        //@ts-ignore
        drawerRef.value.open()
    }

    const handleCreate = () => {
        editId.value = 0
        //@ts-ignore
        resetForm(defaultForm)
        //@ts-ignore
        drawerRef.value.open()
    }

    return {
        drawerRef,
        formRef,
        form,
        editId,
        drawerTitle,
        rules,
        resetForm,
        handleSubmit,
        handleEdit,
        handleCreate,
    }
}