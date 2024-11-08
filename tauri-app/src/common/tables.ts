import { reactive, ref } from "vue";

export const useInitTable = (opt = {}) => {
    let searchForm = null;
    let resetSearchForm = null;
    if(opt.searchForm) {
        searchForm = reactive({ ...opt.searchForm })

        resetSearchForm = () => {
            console.log("is success" + JSON.stringify(searchForm))
            for(const key in opt.searchForm) {
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
        await opt.getList(searchForm).then((res) => {
            if (opt.onGetListSuccess && typeof opt.onGetListSuccess == "function") {
                opt.onGetListSuccess(res)
            } else {
                tableData.value = res
            }
        }).finally(() => {
            loading.value = false
        })
    }

    getData()

    return {
        searchForm,
        tableData,
        total,
        currentPage,
        limit,
        getData,
    }
}