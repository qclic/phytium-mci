
def output_int_mask():
    # 定义各个位的值
    FSDIF_INT_RE_BIT = 1 << 1
    FSDIF_INT_CMD_BIT = 1 << 2
    FSDIF_INT_RCRC_BIT = 1 << 6
    FSDIF_INT_RTO_BIT = 1 << 8
    FSDIF_INT_HTO_BIT = 1 << 10
    FSDIF_INT_HLE_BIT = 1 << 12

    FSDIF_INT_DTO_BIT = 1 << 3
    FSDIF_INT_DCRC_BIT = 1 << 7
    FSDIF_INT_DRTO_BIT = 1 << 9
    FSDIF_INT_SBE_BCI_BIT = 1 << 13

    # 计算 FSDIF_INTS_CMD_MASK
    FSDIF_INTS_CMD_MASK = (
        FSDIF_INT_RE_BIT | FSDIF_INT_CMD_BIT | FSDIF_INT_RCRC_BIT |
        FSDIF_INT_RTO_BIT | FSDIF_INT_HTO_BIT | FSDIF_INT_HLE_BIT
    )

    # 计算 FSDIF_INTS_DATA_MASK
    FSDIF_INTS_DATA_MASK = (
        FSDIF_INT_DTO_BIT | FSDIF_INT_DCRC_BIT | FSDIF_INT_DRTO_BIT |
        FSDIF_INT_SBE_BCI_BIT
    )

    # 输出结果
    print(f"FSDIF_INTS_CMD_MASK: {FSDIF_INTS_CMD_MASK:#x}")
    print(f"FSDIF_INTS_DATA_MASK: {FSDIF_INTS_DATA_MASK:#x}")

def dmac_int_ena():
    # 定义各个位的值
    FSDIF_DMAC_INT_ENA_FBE = 1 << 2
    FSDIF_DMAC_INT_ENA_DU = 1 << 4
    FSDIF_DMAC_INT_ENA_NIS = 1 << 8
    FSDIF_DMAC_INT_ENA_AIS = 1 << 9

    # 计算 FSDIF_DMAC_INTS_MASK
    FSDIF_DMAC_INTS_MASK = (
        FSDIF_DMAC_INT_ENA_FBE |
        FSDIF_DMAC_INT_ENA_DU |
        FSDIF_DMAC_INT_ENA_NIS |
        FSDIF_DMAC_INT_ENA_AIS
    )

    # 输出结果
    print(f"FSDIF_DMAC_INTS_MASK: {FSDIF_DMAC_INTS_MASK:#x}")
    
if __name__ == "__main__":
    dmac_int_ena()
    