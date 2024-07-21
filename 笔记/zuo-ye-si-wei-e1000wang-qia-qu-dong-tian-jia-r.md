# 作业四：为e1000网卡驱动添加remove代码   
# 找到默认 e1000 网卡的驱动实现   
源代码位于路径：`/linux/drivers/net/ethernet/intel/e1000/ `   
> e1000_main.c    

```
static struct pci_driver e1000_driver = {
	.name     = e1000_driver_name,
	.id_table = e1000_pci_tbl,
	.probe    = e1000_probe,
	.remove   = e1000_remove,	// 函数指针
	.driver = {
		.pm = &e1000_pm_ops,
	},
	.shutdown = e1000_shutdown,
	.err_handler = &e1000_err_handler
};
```
这里定义了一个类型为 `pci\_driver` **静态结构体**，它在整个程序运行期间都存在，并且只能在定义它的源文件中访问。其中 `remove` 字段是一个函数指针，指向 `e1000\_remove` 函数：   
```
/**
 * e1000_remove - Device Removal Routine
 * @pdev: PCI device information struct
 *
 * e1000_remove is called by the PCI subsystem to alert the driver
 * that it should release a PCI device. That could be caused by a
 * Hot-Plug event, or because the driver is going to be removed from
 * memory.
 **/
static void e1000_remove(struct pci_dev *pdev)
{
	// 获取网络设备和适配器
	struct net_device *netdev = pci_get_drvdata(pdev);
	struct e1000_adapter *adapter = netdev_priv(netdev);
	struct e1000_hw *hw = &adapter->hw;
	bool disable_dev;
	
	// 停止适配器，关闭网络接口
	e1000_down_and_stop(adapter);
	// 释放适配器的资源
	e1000_release_manageability(adapter);
	
	// 注销网络设备，解除注册
	unregister_netdev(netdev);
	
	// 对 PHY 硬件进行重置
	e1000_phy_hw_reset(hw);
	
	// 释放传输和接收环的内存
	kfree(adapter->tx_ring);
	kfree(adapter->rx_ring);
	
	// 解除映射特定的 MDIO 基地址（仅在 mac 类型为 e1000_ce4100 时）
	if (hw->mac_type == e1000_ce4100)
		iounmap(hw->ce4100_gbe_mdio_base_virt);
	// 解除映射硬件地址
	iounmap(hw->hw_addr);
	// 解除映射闪存地址（如果存在）
	if (hw->flash_address)
		iounmap(hw->flash_address);
	// 释放 PCI 设备的选定区域
	pci_release_selected_regions(pdev, adapter->bars);
	
	// 测试并设置适配器标志中的 __E1000_DISABLED 位，如果之前未设置，则返回 true
	disable_dev = !test_and_set_bit(__E1000_DISABLED, &adapter->flags);
	// 禁用 PCI 设备
	free_netdev(netdev);

	if (disable_dev)
		pci_disable_device(pdev);
}
```
这里的 `e1000\_remove` 定义为一个**静态函数。静态函数**的作用域仅限于定义它的源文件。它不能被其他源文件直接调用或访问。这有助于封装函数实现，防止函数名冲突。   
> /include/linux/pci.h include/linux/pci.h   

```
 * @remove:	The remove() function gets called whenever a device
 *		being handled by this driver is removed (either during
 *		deregistration of the driver or when it's manually
 *		pulled out of a hot-pluggable slot).
 *		The remove function always gets called from process
 *		context, so it can sleep.

struct pci_driver {
	struct list_head	node;
	const char		*name;
	const struct pci_device_id *id_table;	/* Must be non-NULL for probe to be called */
	int  (*probe)(struct pci_dev *dev, const struct pci_device_id *id);	/* New device inserted */
	void (*remove)(struct pci_dev *dev);	/* Device removed (NULL if not a hot-plug capable driver) */
	int  (*suspend)(struct pci_dev *dev, pm_message_t state);	/* Device suspended */
	int  (*resume)(struct pci_dev *dev);	/* Device woken up */
	void (*shutdown)(struct pci_dev *dev);
	int  (*sriov_configure)(struct pci_dev *dev, int num_vfs); /* On PF */
	int  (*sriov_set_msix_vec_count)(struct pci_dev *vf, int msix_vec_count); /* On PF */
	u32  (*sriov_get_vf_total_msix)(struct pci_dev *pf);
	const struct pci_error_handlers *err_handler;
	const struct attribute_group **groups;
	const struct attribute_group **dev_groups;
	struct device_driver	driver;
	struct pci_dynids	dynids;
	bool driver_managed_dma;
};

```
```
/* The pci_dev structure describes PCI devices */
struct pci_dev {
	struct list_head bus_list;	/* Node in per-bus list */
	struct pci_bus	*bus;		/* Bus this device is on */
	struct pci_bus	*subordinate;	/* Bus this device bridges to */

	void		*sysdata;	/* Hook for sys-specific extension */
	struct proc_dir_entry *procent;	/* Device entry in /proc/bus/pci */
	struct pci_slot	*slot;		/* Physical slot this device is in */

	unsigned int	devfn;		/* Encoded device & function index */
	unsigned short	vendor;
	unsigned short	device;
	unsigned short	subsystem_vendor;
	unsigned short	subsystem_device;
	unsigned int	class;		/* 3 bytes: (base,sub,prog-if) */
	u8		revision;	/* PCI revision, low byte of class word */
	u8		hdr_type;	/* PCI header type (`multi' flag masked out) */
#ifdef CONFIG_PCIEAER
	u16		aer_cap;	/* AER capability offset */
	struct aer_stats *aer_stats;	/* AER stats for this device */
#endif
#ifdef CONFIG_PCIEPORTBUS
	struct rcec_ea	*rcec_ea;	/* RCEC cached endpoint association */
	struct pci_dev  *rcec;          /* Associated RCEC device */
#endif
	u32		devcap;		/* PCIe Device Capabilities */
	u8		pcie_cap;	/* PCIe capability offset */
	u8		msi_cap;	/* MSI capability offset */
	u8		msix_cap;	/* MSI-X capability offset */
	u8		pcie_mpss:3;	/* PCIe Max Payload Size Supported */
	u8		rom_base_reg;	/* Config register controlling ROM */
	u8		pin;		/* Interrupt pin this device uses */
	u16		pcie_flags_reg;	/* Cached PCIe Capabilities Register */
	unsigned long	*dma_alias_mask;/* Mask of enabled devfn aliases */

	struct pci_driver *driver;	/* Driver bound to this device */
	u64		dma_mask;	/* Mask of the bits of bus address this
					   device implements.  Normally this is
					   0xffffffff.  You only need to change
					   this if your device has broken DMA
					   or supports 64-bit transfers.  */

	struct device_dma_parameters dma_parms;

	pci_power_t	current_state;	/* Current operating state. In ACPI,
					   this is D0-D3, D0 being fully
					   functional, and D3 being off. */
	unsigned int	imm_ready:1;	/* Supports Immediate Readiness */
	u8		pm_cap;		/* PM capability offset */
	unsigned int	pme_support:5;	/* Bitmask of states from which PME#
					   can be generated */
	unsigned int	pme_poll:1;	/* Poll device's PME status bit */
	unsigned int	d1_support:1;	/* Low power state D1 is supported */
	unsigned int	d2_support:1;	/* Low power state D2 is supported */
	unsigned int	no_d1d2:1;	/* D1 and D2 are forbidden */
	unsigned int	no_d3cold:1;	/* D3cold is forbidden */
	unsigned int	bridge_d3:1;	/* Allow D3 for bridge */
	unsigned int	d3cold_allowed:1;	/* D3cold is allowed by user */
	unsigned int	mmio_always_on:1;	/* Disallow turning off io/mem
						   decoding during BAR sizing */
	unsigned int	wakeup_prepared:1;
	unsigned int	skip_bus_pm:1;	/* Internal: Skip bus-level PM */
	unsigned int	ignore_hotplug:1;	/* Ignore hotplug events */
	unsigned int	hotplug_user_indicators:1; /* SlotCtl indicators
						      controlled exclusively by
						      user sysfs */
	unsigned int	clear_retrain_link:1;	/* Need to clear Retrain Link
						   bit manually */
	unsigned int	d3hot_delay;	/* D3hot->D0 transition time in ms */
	unsigned int	d3cold_delay;	/* D3cold->D0 transition time in ms */

#ifdef CONFIG_PCIEASPM
	struct pcie_link_state	*link_state;	/* ASPM link state */
	unsigned int	ltr_path:1;	/* Latency Tolerance Reporting
					   supported from root to here */
	u16		l1ss;		/* L1SS Capability pointer */
#endif
	unsigned int	pasid_no_tlp:1;		/* PASID works without TLP Prefix */
	unsigned int	eetlp_prefix_path:1;	/* End-to-End TLP Prefix */

	pci_channel_state_t error_state;	/* Current connectivity state */
	struct device	dev;			/* Generic device interface */

	int		cfg_size;		/* Size of config space */

	/*
	 * Instead of touching interrupt line and base address registers
	 * directly, use the values stored here. They might be different!
	 */
	unsigned int	irq;
	struct resource resource[DEVICE_COUNT_RESOURCE]; /* I/O and memory regions + expansion ROMs */

	bool		match_driver;		/* Skip attaching driver */

	unsigned int	transparent:1;		/* Subtractive decode bridge */
	unsigned int	io_window:1;		/* Bridge has I/O window */
	unsigned int	pref_window:1;		/* Bridge has pref mem window */
	unsigned int	pref_64_window:1;	/* Pref mem window is 64-bit */
	unsigned int	multifunction:1;	/* Multi-function device */

	unsigned int	is_busmaster:1;		/* Is busmaster */
	unsigned int	no_msi:1;		/* May not use MSI */
	unsigned int	no_64bit_msi:1;		/* May only use 32-bit MSIs */
	unsigned int	block_cfg_access:1;	/* Config space access blocked */
	unsigned int	broken_parity_status:1;	/* Generates false positive parity */
	unsigned int	irq_reroute_variant:2;	/* Needs IRQ rerouting variant */
	unsigned int	msi_enabled:1;
	unsigned int	msix_enabled:1;
	unsigned int	ari_enabled:1;		/* ARI forwarding */
	unsigned int	ats_enabled:1;		/* Address Translation Svc */
	unsigned int	pasid_enabled:1;	/* Process Address Space ID */
	unsigned int	pri_enabled:1;		/* Page Request Interface */
	unsigned int	is_managed:1;		/* Managed via devres */
	unsigned int	is_msi_managed:1;	/* MSI release via devres installed */
	unsigned int	needs_freset:1;		/* Requires fundamental reset */
	unsigned int	state_saved:1;
	unsigned int	is_physfn:1;
	unsigned int	is_virtfn:1;
	unsigned int	is_hotplug_bridge:1;
	unsigned int	shpc_managed:1;		/* SHPC owned by shpchp */
	unsigned int	is_thunderbolt:1;	/* Thunderbolt controller */
	/*
	 * Devices marked being untrusted are the ones that can potentially
	 * execute DMA attacks and similar. They are typically connected
	 * through external ports such as Thunderbolt but not limited to
	 * that. When an IOMMU is enabled they should be getting full
	 * mappings to make sure they cannot access arbitrary memory.
	 */
	unsigned int	untrusted:1;
	/*
	 * Info from the platform, e.g., ACPI or device tree, may mark a
	 * device as "external-facing".  An external-facing device is
	 * itself internal but devices downstream from it are external.
	 */
	unsigned int	external_facing:1;
	unsigned int	broken_intx_masking:1;	/* INTx masking can't be used */
	unsigned int	io_window_1k:1;		/* Intel bridge 1K I/O windows */
	unsigned int	irq_managed:1;
	unsigned int	non_compliant_bars:1;	/* Broken BARs; ignore them */
	unsigned int	is_probed:1;		/* Device probing in progress */
	unsigned int	link_active_reporting:1;/* Device capable of reporting link active */
	unsigned int	no_vf_scan:1;		/* Don't scan for VFs after IOV enablement */
	unsigned int	no_command_memory:1;	/* No PCI_COMMAND_MEMORY */
	unsigned int	rom_bar_overlap:1;	/* ROM BAR disable broken */
	pci_dev_flags_t dev_flags;
	atomic_t	enable_cnt;	/* pci_enable_device has been called */

	u32		saved_config_space[16]; /* Config space saved at suspend time */
	struct hlist_head saved_cap_space;
	int		rom_attr_enabled;	/* Display of ROM attribute enabled? */
	struct bin_attribute *res_attr[DEVICE_COUNT_RESOURCE]; /* sysfs file for resources */
	struct bin_attribute *res_attr_wc[DEVICE_COUNT_RESOURCE]; /* sysfs file for WC mapping of resources */

#ifdef CONFIG_HOTPLUG_PCI_PCIE
	unsigned int	broken_cmd_compl:1;	/* No compl for some cmds */
#endif
#ifdef CONFIG_PCIE_PTM
	u16		ptm_cap;		/* PTM Capability */
	unsigned int	ptm_root:1;
	unsigned int	ptm_enabled:1;
	u8		ptm_granularity;
#endif
#ifdef CONFIG_PCI_MSI
	void __iomem	*msix_base;
	raw_spinlock_t	msi_lock;
#endif
	struct pci_vpd	vpd;
#ifdef CONFIG_PCIE_DPC
	u16		dpc_cap;
	unsigned int	dpc_rp_extensions:1;
	u8		dpc_rp_log_size;
#endif
#ifdef CONFIG_PCI_ATS
	union {
		struct pci_sriov	*sriov;		/* PF: SR-IOV info */
		struct pci_dev		*physfn;	/* VF: related PF */
	};
	u16		ats_cap;	/* ATS Capability offset */
	u8		ats_stu;	/* ATS Smallest Translation Unit */
#endif
#ifdef CONFIG_PCI_PRI
	u16		pri_cap;	/* PRI Capability offset */
	u32		pri_reqs_alloc; /* Number of PRI requests allocated */
	unsigned int	pasid_required:1; /* PRG Response PASID Required */
#endif
#ifdef CONFIG_PCI_PASID
	u16		pasid_cap;	/* PASID Capability offset */
	u16		pasid_features;
#endif
#ifdef CONFIG_PCI_P2PDMA
	struct pci_p2pdma __rcu *p2pdma;
#endif
	u16		acs_cap;	/* ACS Capability offset */
	phys_addr_t	rom;		/* Physical address if not from BAR */
	size_t		romlen;		/* Length if not from BAR */
	/*
	 * Driver name to force a match.  Do not set directly, because core
	 * frees it.  Use driver_set_override() to set or clear it.
	 */
	const char	*driver_override;

	unsigned long	priv_flags;	/* Private flags for the PCI driver */

	/* These methods index pci_reset_fn_methods[] */
	u8 reset_methods[PCI_NUM_RESET_METHODS]; /* In priority order */
};
```
# rust 实现   
> pci::Driver 特征的实现   

```
pub trait Driver {
    /// Data stored on device by driver.
    ///
    /// Corresponds to the data set or retrieved via the kernel's
    /// `pci_{set,get}_drvdata()` functions.
    ///
    /// Require that `Data` implements `PointerWrapper`. We guarantee to
    /// never move the underlying wrapped data structure.
    type Data: PointerWrapper + Send + Sync + driver::DeviceRemoval = ();

    /// The type holding information about each device id supported by the driver.
    type IdInfo: 'static = ();

    /// The table of device ids supported by the driver.
    const ID_TABLE: driver::IdTable<'static, DeviceId, Self::IdInfo>;

    /// PCI driver probe.
    ///
    /// Called when a new platform device is added or discovered.
    /// Implementers should attempt to initialize the device here.
    fn probe(dev: &mut Device, id: Option<&Self::IdInfo>) -> Result<Self::Data>;

    /// PCI driver remove.
    ///
    /// Called when a platform device is removed.
    /// Implementers should prepare the device for complete removal here.
    fn remove(_data: &Self::Data);
}

```
> 设备初始化函数：   

```
    fn probe(dev: &mut pci::Device, id: core::option::Option<&Self::IdInfo>) -> Result<Self::Data> {
        pr_info!("Rust for linux e1000 driver demo (probe): {:?}\n", id);

        // 只支持QEMU的82540EM芯片.
        
        
        // 于选择设备的基地址寄存器(BAR)，返回一个掩码(mask)，指示哪些BAR被选中
		let bars = dev.select_bars((bindings::IORESOURCE_MEM | bindings::IORESOURCE_IO) as u64);

        // the underlying will call `pci_enable_device()`. the R4L framework doesn't support `pci_enable_device_memory()` now.
        // 启用设备，相当于调用pci_enable_device()
		dev.enable_device()?;

        // ask the os to reserve the physical memory region of the selected bars.
        // 请求操作系统保留所选BAR的物理内存区域
		dev.request_selected_regions(bars, c_str!("e1000 reserved memory"))?;

        // set device to master mode.
        dev.set_master();

        // get resource(memory range) provided by BAR0
		// 获取由BAR0提供的资源(内存范围)
        let mem_res = dev.iter_resource().next().ok_or(kernel::error::code::EIO)?;
		// 获取IO端口资源
        let io_res = dev.iter_resource().skip(1).find(|r:&Resource|r.check_flags(bindings::IORESOURCE_IO)).ok_or(kernel::error::code::EIO)?;

        // TODO pci_save_state(pdev); not supported by crate now, only have raw C bindings.

        // alloc new ethernet device, this line represent the `alloc_etherdev()` and `SET_NETDEV_DEV()` in C version.
		// 分配新的以太网设备，相当于alloc_etherdev()和SET_NETDEV_DEV()
        let mut netdev_reg = net::Registration::<NetDevice>::try_new(dev)?;
        let netdev = netdev_reg.dev_get();

        // map device registers' hardware address to logical address so the kernel driver can access it.
		// 将设备寄存器的硬件地址映射到逻辑地址
        let mem_addr = Arc::try_new(dev.map_resource(&mem_res, mem_res.len())?)?;

        // get the io-port based address
		// 获取基于IO端口的地址
        let io_addr = Arc::try_new(pci::IoPort::try_new(&io_res)?)?;



        // TODO implement C version `e1000_init_hw_struct()`

        // only pci-x need 64-bit, to simplify code, hardcode 32-bit for now.
        // 设置DMA一致性掩码为32位
		dma::set_coherent_mask(dev, 0xFFFFFFFF)?;

        // TODO ethtool support here.

        // Enable napi, the R4L will call `netif_napi_add_weight()`, the origin C version calls `netif_napi_add`
        // 启用NAPI（新API），用于高效处理网络数据包
		let napi = net::NapiAdapter::<NapiHandler>::add_weight(&netdev, 64)?;


        // TODO implement C version `e1000_sw_init()`

        // TODO a lot of feature flags are assigned here in the C code, skip them for now.
		// 初始化硬件操作对象，并调用e1000_reset_hw()重置硬件
        let e1000_hw_ops = E1000Ops {
            mem_addr: Arc::clone(&mem_addr),
            io_addr: Arc::clone(&io_addr),
        };
        e1000_hw_ops.e1000_reset_hw()?;


        // TODO: the MAC address is hardcoded here, should be read out from EEPROM later.
		// 设置硬件地址（MAC地址）
        netdev.eth_hw_addr_set(&MAC_HWADDR);

        // TODO: Some background tasks and Wake on LAN are not supported now.
		
		// 获取设备的IRQ号
        let irq = dev.irq();

        // 创建通用设备对象
		let common_dev = device::Device::from_dev(dev);

        // 关闭网络接口的载波
		netdev.netif_carrier_off();


		//初始化发送和接收环缓冲区的自旋锁
        // SAFETY: `spinlock_init` is called below.
        let mut tx_ring = unsafe{SpinLock::new(None)};
        let mut rx_ring = unsafe{SpinLock::new(None)};
        // SAFETY: We don't move `tx_ring` and `rx_ring`.
        kernel::spinlock_init!(unsafe{Pin::new_unchecked(&mut tx_ring)}, "tx_ring");
        kernel::spinlock_init!(unsafe{Pin::new_unchecked(&mut rx_ring)}, "rx_ring");

		// 注册网络设备的私有数据
        netdev_reg.register(Box::try_new(
            NetDevicePrvData {
                dev: Arc::try_new(common_dev)?,
                e1000_hw_ops: Arc::try_new(e1000_hw_ops)?,
                napi: napi.into(),
                tx_ring,
                rx_ring,
                irq,
                _irq_handler: AtomicPtr::new(core::ptr::null_mut()),
            }
        )?)?;

        
		// 返回一个包含网络设备注册数据的盒子（Box）
        Ok(Box::try_new(
            E1000DrvPrvData{
                // Must hold this registration, or the device will be removed.
                _netdev_reg: netdev_reg,
            }
        )?)
    }
```
# 移出内核模块调用的函数   
```
[   73.640100] r4l_e1000_demo: Rust for linux e1000 driver demo (exit)
[   73.640654] r4l_e1000_demo: Rust for linux e1000 driver demo (remove)
[   73.640779] r4l_e1000_demo: Rust for linux e1000 driver demo (device_remove)
[   73.644030] r4l_e1000_demo: Rust for linux e1000 driver demo (net device get_stats64)
```
   
