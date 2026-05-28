

     _ram_mpu_ro_region_start = ORIGIN(RAM);



/* Linker section for memory region RAM for TEXT section  */

	SECTION_PROLOGUE(.ram_text_reloc,,)
        {
                __ram_text_reloc_start = .;
                
                KEEP(*flash_clock_setup.c.obj(".rel.text.flexspi_clock_safe_config"))

                KEEP(*flash_clock_setup.c.obj(".text.flexspi_clock_safe_config"))

                KEEP(*flash_clock_setup.c.obj(".text.flexspi_setup_clock"))

#if 0
                . = ALIGN(0);
#else
                MPU_ALIGN(__ram_text_reloc_size);
#endif
                __ram_text_reloc_end = .;
	} 
#ifdef CONFIG_XIP
GROUP_DATA_LINK_IN(RAM , ROMABLE_REGION)
#else
GROUP_DATA_LINK_IN(RAM , RAM )
#endif

        __ram_text_reloc_size = __ram_text_reloc_end - __ram_text_reloc_start;

        __ram_text_rom_start = LOADADDR(.ram_text_reloc);


    _ram_mpu_ro_region_end = .;

