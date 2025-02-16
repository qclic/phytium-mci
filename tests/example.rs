//* 输出所有的nodes */
fdt.all_nodes().for_each(|node| {
    node.compatibles().for_each(|compatible| {
        info!("node compatible: {}", compatible);
    });
    node.reg().map(|reg| {
        reg.for_each(|r| {
            info!("node reg: {:#x}", r.address);
            if let Some(size) = r.size {
                info!("node reg size: {:#x}", size);
            }
        });
    });
    debug!("\n")
});
💡 36.853s [test:34] node compatible: phytium,pe2204
🐛 36.860s [test:44] 

💡 36.864s [test:38] node reg: 0x2000000000
💡 36.870s [test:40] node reg size: 0x80000000
🐛 36.876s [test:44] 

🐛 36.880s [test:44]

💡 36.884s [test:34] node compatible: arm,psci-1.0
🐛 36.890s [test:44] 

🐛 36.895s [test:44]

💡 36.899s [test:34] node compatible: arm,scmi
🐛 36.905s [test:44] 

💡 36.909s [test:38] node reg: 0x13
🐛 36.914s [test:44] 

💡 36.918s [test:38] node reg: 0x15
🐛 36.923s [test:44] 

💡 36.928s [test:34] node compatible: linaro,optee-tz
🐛 36.934s [test:44] 

🐛 36.938s [test:44] 

🐛 36.943s [test:44]

🐛 36.947s [test:44]

🐛 36.951s [test:44]

🐛 36.955s [test:44] 

🐛 36.959s [test:44]

🐛 36.963s [test:44]

🐛 36.968s [test:44]

🐛 36.972s [test:44] 

🐛 36.976s [test:44]

🐛 36.980s [test:44]

🐛 36.984s [test:44]

💡 36.988s [test:34] node compatible: phytium,ftc310
💡 36.995s [test:34] node compatible: arm,armv8
💡 37.001s [test:38] node reg: 0x200
🐛 37.006s [test:44] 

💡 37.011s [test:34] node compatible: phytium,ftc310
💡 37.017s [test:34] node compatible: arm,armv8
💡 37.023s [test:38] node reg: 0x201
🐛 37.028s [test:44] 

💡 37.033s [test:34] node compatible: phytium,ftc664
💡 37.039s [test:34] node compatible: arm,armv8
💡 37.045s [test:38] node reg: 0x0
🐛 37.050s [test:44] 

💡 37.055s [test:34] node compatible: phytium,ftc664
💡 37.061s [test:34] node compatible: arm,armv8
💡 37.067s [test:38] node reg: 0x100
🐛 37.073s [test:44] 

💡 37.077s [test:34] node compatible: arm,gic-v3
💡 37.083s [test:38] node reg: 0x30800000
💡 37.089s [test:40] node reg size: 0x20000
💡 37.094s [test:38] node reg: 0x30880000
💡 37.100s [test:40] node reg size: 0x80000
💡 37.106s [test:38] node reg: 0x30840000
💡 37.112s [test:40] node reg size: 0x10000
💡 37.117s [test:38] node reg: 0x30850000
💡 37.123s [test:40] node reg size: 0x10000
💡 37.129s [test:38] node reg: 0x30860000
💡 37.134s [test:40] node reg size: 0x10000
🐛 37.140s [test:44] 

💡 37.144s [test:34] node compatible: arm,gic-v3-its
💡 37.151s [test:38] node reg: 0x30820000
💡 37.157s [test:40] node reg size: 0x20000
🐛 37.162s [test:44] 

💡 37.167s [test:34] node compatible: arm,armv8-pmuv3
🐛 37.173s [test:44] 

💡 37.178s [test:34] node compatible: arm,armv8-timer
🐛 37.184s [test:44] 

🐛 37.188s [test:44] 

💡 37.193s [test:34] node compatible: fixed-clock
🐛 37.199s [test:44] 

💡 37.203s [test:34] node compatible: fixed-clock
🐛 37.209s [test:44] 

💡 37.214s [test:34] node compatible: fixed-clock
🐛 37.220s [test:44] 

💡 37.224s [test:34] node compatible: fixed-clock
🐛 37.230s [test:44] 

💡 37.235s [test:34] node compatible: fixed-clock
🐛 37.241s [test:44] 

💡 37.245s [test:34] node compatible: fixed-clock
🐛 37.251s [test:44] 

💡 37.256s [test:34] node compatible: fixed-clock
🐛 37.262s [test:44] 

💡 37.266s [test:34] node compatible: fixed-clock
🐛 37.272s [test:44] 

💡 37.277s [test:34] node compatible: arm,smmu-v3
💡 37.283s [test:38] node reg: 0x30000000
💡 37.288s [test:40] node reg size: 0x800000
🐛 37.294s [test:44] 

💡 37.299s [test:34] node compatible: simple-bus
🐛 37.305s [test:44] 

💡 37.309s [test:34] node compatible: phytium,mci
💡 37.315s [test:38] node reg: 0x28000000
💡 37.321s [test:40] node reg size: 0x1000
🐛 37.327s [test:44] 

💡 37.331s [test:34] node compatible: phytium,mci
💡 37.337s [test:38] node reg: 0x28001000
💡 37.343s [test:40] node reg size: 0x1000
🐛 37.349s [test:44] 

💡 37.353s [test:34] node compatible: phytium,nfc
💡 37.359s [test:38] node reg: 0x28002000
💡 37.365s [test:40] node reg size: 0x1000
🐛 37.370s [test:44] 

💡 37.375s [test:34] node compatible: phytium,ddma
💡 37.381s [test:38] node reg: 0x28003000
💡 37.387s [test:40] node reg size: 0x1000
🐛 37.392s [test:44] 

💡 37.397s [test:34] node compatible: phytium,ddma
💡 37.403s [test:38] node reg: 0x28004000
💡 37.409s [test:40] node reg size: 0x1000
🐛 37.414s [test:44] 

💡 37.419s [test:34] node compatible: phytium,qspi-nor
💡 37.425s [test:38] node reg: 0x28008000
💡 37.431s [test:40] node reg size: 0x1000
💡 37.437s [test:38] node reg: 0x0
💡 37.442s [test:40] node reg size: 0xfffffff
🐛 37.448s [test:44] 

💡 37.452s [test:34] node compatible: jedec,spi-nor
💡 37.458s [test:38] node reg: 0x0
🐛 37.463s [test:44] 

💡 37.468s [test:34] node compatible: arm,pl011
💡 37.474s [test:34] node compatible: arm,primecell
💡 37.480s [test:38] node reg: 0x2800c000
💡 37.486s [test:40] node reg size: 0x1000
🐛 37.492s [test:44] 

💡 37.496s [test:34] node compatible: arm,pl011
💡 37.502s [test:34] node compatible: arm,primecell
💡 37.508s [test:38] node reg: 0x2800d000
💡 37.514s [test:40] node reg size: 0x1000
🐛 37.520s [test:44] 

💡 37.524s [test:34] node compatible: arm,pl011
💡 37.530s [test:34] node compatible: arm,primecell
💡 37.537s [test:38] node reg: 0x2800e000
💡 37.542s [test:40] node reg size: 0x1000
🐛 37.548s [test:44] 

💡 37.552s [test:34] node compatible: arm,pl011
💡 37.558s [test:34] node compatible: arm,primecell
💡 37.565s [test:38] node reg: 0x2800f000
💡 37.571s [test:40] node reg size: 0x1000
🐛 37.576s [test:44] 

💡 37.580s [test:34] node compatible: simple-mfd
💡 37.587s [test:34] node compatible: syscon
💡 37.593s [test:38] node reg: 0x28010000
💡 37.598s [test:40] node reg size: 0x1000
🐛 37.604s [test:44] 

💡 37.608s [test:34] node compatible: phytium,kcs-bmc
💡 37.615s [test:38] node reg: 0x28010024
💡 37.620s [test:40] node reg size: 0x1
💡 37.626s [test:38] node reg: 0x28010030
💡 37.632s [test:40] node reg size: 0x1
💡 37.637s [test:38] node reg: 0x2801003c
💡 37.643s [test:40] node reg size: 0x1
🐛 37.648s [test:44] 

💡 37.652s [test:34] node compatible: phytium,kcs-bmc
💡 37.659s [test:38] node reg: 0x28010028
💡 37.665s [test:40] node reg size: 0x1
💡 37.670s [test:38] node reg: 0x28010034
💡 37.676s [test:40] node reg size: 0x1
💡 37.681s [test:38] node reg: 0x28010040
💡 37.687s [test:40] node reg size: 0x1
🐛 37.692s [test:44] 

💡 37.696s [test:34] node compatible: phytium,kcs-bmc
💡 37.703s [test:38] node reg: 0x2801002c
💡 37.709s [test:40] node reg size: 0x1
💡 37.714s [test:38] node reg: 0x28010038
💡 37.720s [test:40] node reg size: 0x1
💡 37.725s [test:38] node reg: 0x28010044
💡 37.731s [test:40] node reg size: 0x1
🐛 37.736s [test:44] 

💡 37.741s [test:34] node compatible: phytium,kcs-bmc
💡 37.747s [test:38] node reg: 0x2801008c
💡 37.753s [test:40] node reg size: 0x1
💡 37.758s [test:38] node reg: 0x28010090
💡 37.764s [test:40] node reg size: 0x1
💡 37.770s [test:38] node reg: 0x28010094
💡 37.775s [test:40] node reg size: 0x1
🐛 37.781s [test:44] 

💡 37.785s [test:34] node compatible: phytium,bt-bmc
💡 37.791s [test:38] node reg: 0x28010048
💡 37.797s [test:40] node reg size: 0x20
🐛 37.803s [test:44] 

💡 37.807s [test:34] node compatible: phytium,gpio
💡 37.813s [test:38] node reg: 0x28034000
💡 37.819s [test:40] node reg size: 0x1000
🐛 37.825s [test:44] 

💡 37.829s [test:34] node compatible: phytium,gpio-port
💡 37.836s [test:38] node reg: 0x0
🐛 37.841s [test:44] 

💡 37.845s [test:34] node compatible: phytium,gpio
💡 37.851s [test:38] node reg: 0x28035000
💡 37.857s [test:40] node reg size: 0x1000
🐛 37.863s [test:44] 

💡 37.867s [test:34] node compatible: phytium,gpio-port
💡 37.874s [test:38] node reg: 0x0
🐛 37.879s [test:44] 

💡 37.883s [test:34] node compatible: phytium,gpio
💡 37.889s [test:38] node reg: 0x28036000
💡 37.895s [test:40] node reg size: 0x1000
🐛 37.901s [test:44] 

💡 37.905s [test:34] node compatible: phytium,gpio-port
💡 37.912s [test:38] node reg: 0x0
🐛 37.917s [test:44] 

💡 37.921s [test:34] node compatible: phytium,gpio
💡 37.927s [test:38] node reg: 0x28037000
💡 37.933s [test:40] node reg size: 0x1000
🐛 37.939s [test:44] 

💡 37.943s [test:34] node compatible: phytium,gpio-port
💡 37.950s [test:38] node reg: 0x0
🐛 37.955s [test:44] 

💡 37.959s [test:34] node compatible: phytium,gpio
💡 37.965s [test:38] node reg: 0x28038000
💡 37.971s [test:40] node reg size: 0x1000
🐛 37.977s [test:44] 

💡 37.981s [test:34] node compatible: phytium,gpio-port
💡 37.988s [test:38] node reg: 0x0
🐛 37.993s [test:44] 

💡 37.997s [test:34] node compatible: phytium,gpio
💡 38.003s [test:38] node reg: 0x28039000
💡 38.009s [test:40] node reg size: 0x1000
🐛 38.015s [test:44] 

💡 38.019s [test:34] node compatible: phytium,gpio-port
💡 38.026s [test:38] node reg: 0x0
🐛 38.031s [test:44] 

💡 38.035s [test:34] node compatible: phytium,spi
💡 38.041s [test:38] node reg: 0x2803a000
💡 38.047s [test:40] node reg size: 0x1000
🐛 38.053s [test:44] 

💡 38.057s [test:34] node compatible: phytium,spi
💡 38.063s [test:38] node reg: 0x2803b000
💡 38.069s [test:40] node reg size: 0x1000
🐛 38.074s [test:44] 

💡 38.079s [test:34] node compatible: phytium,spi
💡 38.085s [test:38] node reg: 0x2803c000
💡 38.091s [test:40] node reg size: 0x1000
🐛 38.096s [test:44] 

💡 38.101s [test:34] node compatible: phytium,spi
💡 38.107s [test:38] node reg: 0x2803d000
💡 38.112s [test:40] node reg size: 0x1000
🐛 38.118s [test:44] 

💡 38.122s [test:34] node compatible: arm,sbsa-gwdt
💡 38.129s [test:38] node reg: 0x28041000
💡 38.135s [test:40] node reg size: 0x1000
💡 38.140s [test:38] node reg: 0x28040000
💡 38.146s [test:40] node reg size: 0x1000
🐛 38.152s [test:44] 

💡 38.156s [test:34] node compatible: arm,sbsa-gwdt
💡 38.162s [test:38] node reg: 0x28043000
💡 38.168s [test:40] node reg size: 0x1000
💡 38.174s [test:38] node reg: 0x28042000
💡 38.179s [test:40] node reg size: 0x1000
🐛 38.185s [test:44] 

💡 38.189s [test:34] node compatible: phytium,pwm
💡 38.196s [test:38] node reg: 0x2804a000
💡 38.201s [test:40] node reg size: 0x1000
🐛 38.207s [test:44] 

💡 38.211s [test:34] node compatible: phytium,pwm
💡 38.217s [test:38] node reg: 0x2804b000
💡 38.223s [test:40] node reg size: 0x1000
🐛 38.229s [test:44] 

💡 38.233s [test:34] node compatible: phytium,tacho
💡 38.239s [test:38] node reg: 0x28054000
💡 38.245s [test:40] node reg size: 0x1000
🐛 38.251s [test:44] 

💡 38.255s [test:34] node compatible: phytium,tacho
💡 38.262s [test:38] node reg: 0x28055000
💡 38.267s [test:40] node reg size: 0x1000
🐛 38.273s [test:44] 

💡 38.277s [test:34] node compatible: phytium,tacho
💡 38.284s [test:38] node reg: 0x28056000
💡 38.289s [test:40] node reg size: 0x1000
🐛 38.295s [test:44] 

💡 38.299s [test:34] node compatible: phytium,tacho
💡 38.306s [test:38] node reg: 0x28057000
💡 38.311s [test:40] node reg size: 0x1000
🐛 38.317s [test:44] 

💡 38.321s [test:34] node compatible: phytium,tacho
💡 38.328s [test:38] node reg: 0x28058000
💡 38.333s [test:40] node reg size: 0x1000
🐛 38.339s [test:44] 

💡 38.343s [test:34] node compatible: phytium,tacho
💡 38.350s [test:38] node reg: 0x28059000
💡 38.355s [test:40] node reg size: 0x1000
🐛 38.361s [test:44] 

💡 38.365s [test:34] node compatible: phytium,tacho
💡 38.372s [test:38] node reg: 0x2805a000
💡 38.377s [test:40] node reg size: 0x1000
🐛 38.383s [test:44] 

💡 38.387s [test:34] node compatible: phytium,tacho
💡 38.394s [test:38] node reg: 0x2805b000
💡 38.399s [test:40] node reg size: 0x1000
🐛 38.405s [test:44] 

💡 38.409s [test:34] node compatible: phytium,tacho
💡 38.416s [test:38] node reg: 0x2805c000
💡 38.421s [test:40] node reg size: 0x1000
🐛 38.427s [test:44] 

💡 38.431s [test:34] node compatible: phytium,tacho
💡 38.438s [test:38] node reg: 0x2805d000
💡 38.444s [test:40] node reg size: 0x1000
🐛 38.449s [test:44] 

💡 38.453s [test:34] node compatible: phytium,tacho
💡 38.460s [test:38] node reg: 0x2805e000
💡 38.466s [test:40] node reg size: 0x1000
🐛 38.471s [test:44] 

💡 38.476s [test:34] node compatible: phytium,tacho
💡 38.482s [test:38] node reg: 0x2805f000
💡 38.488s [test:40] node reg size: 0x1000
🐛 38.493s [test:44] 

💡 38.498s [test:34] node compatible: phytium,tacho
💡 38.504s [test:38] node reg: 0x28060000
💡 38.510s [test:40] node reg size: 0x1000
🐛 38.515s [test:44] 

💡 38.520s [test:34] node compatible: phytium,tacho
💡 38.526s [test:38] node reg: 0x28061000
💡 38.532s [test:40] node reg size: 0x1000
🐛 38.537s [test:44] 

💡 38.542s [test:34] node compatible: phytium,tacho
💡 38.548s [test:38] node reg: 0x28062000
💡 38.554s [test:40] node reg size: 0x1000
🐛 38.560s [test:44] 

💡 38.564s [test:34] node compatible: phytium,tacho
💡 38.570s [test:38] node reg: 0x28063000
💡 38.576s [test:40] node reg size: 0x1000
🐛 38.582s [test:44] 

💡 38.586s [test:34] node compatible: phytium,tacho
💡 38.592s [test:38] node reg: 0x28064000
💡 38.598s [test:40] node reg size: 0x1000
🐛 38.604s [test:44] 

💡 38.608s [test:34] node compatible: phytium,tacho
💡 38.614s [test:38] node reg: 0x28065000
💡 38.620s [test:40] node reg size: 0x1000
🐛 38.626s [test:44] 

💡 38.630s [test:34] node compatible: phytium,tacho
💡 38.636s [test:38] node reg: 0x28066000
💡 38.642s [test:40] node reg size: 0x1000
🐛 38.648s [test:44] 

💡 38.652s [test:34] node compatible: phytium,tacho
💡 38.658s [test:38] node reg: 0x28067000
💡 38.664s [test:40] node reg size: 0x1000
🐛 38.670s [test:44] 

💡 38.674s [test:34] node compatible: phytium,tacho
💡 38.680s [test:38] node reg: 0x28068000
💡 38.686s [test:40] node reg size: 0x1000
🐛 38.692s [test:44] 

💡 38.696s [test:34] node compatible: phytium,tacho
💡 38.702s [test:38] node reg: 0x28069000
💡 38.708s [test:40] node reg size: 0x1000
🐛 38.714s [test:44] 

💡 38.718s [test:34] node compatible: phytium,tacho
💡 38.725s [test:38] node reg: 0x2806a000
💡 38.730s [test:40] node reg size: 0x1000
🐛 38.736s [test:44] 

💡 38.740s [test:34] node compatible: phytium,tacho
💡 38.747s [test:38] node reg: 0x2806b000
💡 38.752s [test:40] node reg size: 0x1000
🐛 38.758s [test:44] 

💡 38.762s [test:34] node compatible: phytium,tacho
💡 38.769s [test:38] node reg: 0x2806c000
💡 38.774s [test:40] node reg size: 0x1000
🐛 38.780s [test:44] 

💡 38.784s [test:34] node compatible: phytium,tacho
💡 38.791s [test:38] node reg: 0x2806d000
💡 38.796s [test:40] node reg size: 0x1000
🐛 38.802s [test:44] 

💡 38.806s [test:34] node compatible: phytium,tacho
💡 38.813s [test:38] node reg: 0x2806e000
💡 38.818s [test:40] node reg size: 0x1000
🐛 38.824s [test:44] 

💡 38.828s [test:34] node compatible: phytium,tacho
💡 38.835s [test:38] node reg: 0x2806f000
💡 38.840s [test:40] node reg size: 0x1000
🐛 38.846s [test:44] 

💡 38.850s [test:34] node compatible: phytium,tacho
💡 38.857s [test:38] node reg: 0x28070000
💡 38.862s [test:40] node reg size: 0x1000
🐛 38.868s [test:44] 

💡 38.872s [test:34] node compatible: phytium,tacho
💡 38.879s [test:38] node reg: 0x28071000
💡 38.884s [test:40] node reg size: 0x1000
🐛 38.890s [test:44] 

💡 38.894s [test:34] node compatible: phytium,tacho
💡 38.901s [test:38] node reg: 0x28072000
💡 38.907s [test:40] node reg size: 0x1000
🐛 38.912s [test:44] 

💡 38.916s [test:34] node compatible: phytium,tacho
💡 38.923s [test:38] node reg: 0x28073000
💡 38.929s [test:40] node reg size: 0x1000
🐛 38.934s [test:44] 

💡 38.939s [test:34] node compatible: phytium,tacho
💡 38.945s [test:38] node reg: 0x28074000
💡 38.951s [test:40] node reg size: 0x1000
🐛 38.956s [test:44] 

💡 38.961s [test:34] node compatible: phytium,tacho
💡 38.967s [test:38] node reg: 0x28075000
💡 38.973s [test:40] node reg size: 0x1000
🐛 38.978s [test:44] 

💡 38.983s [test:34] node compatible: phytium,tacho
💡 38.989s [test:38] node reg: 0x28076000
💡 38.995s [test:40] node reg size: 0x1000
🐛 39.000s [test:44] 

💡 39.005s [test:34] node compatible: phytium,tacho
💡 39.011s [test:38] node reg: 0x28077000
💡 39.017s [test:40] node reg size: 0x1000
🐛 39.022s [test:44] 

💡 39.027s [test:34] node compatible: phytium,tacho
💡 39.033s [test:38] node reg: 0x28078000
💡 39.039s [test:40] node reg size: 0x1000
🐛 39.045s [test:44] 

💡 39.049s [test:34] node compatible: phytium,tacho
💡 39.055s [test:38] node reg: 0x28079000
💡 39.061s [test:40] node reg size: 0x1000
🐛 39.067s [test:44] 

💡 39.071s [test:34] node compatible: phytium,usb2
💡 39.077s [test:38] node reg: 0x31800000
💡 39.083s [test:40] node reg size: 0x80000
💡 39.089s [test:38] node reg: 0x31990000
💡 39.094s [test:40] node reg size: 0x10000
🐛 39.100s [test:44] 

💡 39.104s [test:34] node compatible: phytium,usb2
💡 39.111s [test:38] node reg: 0x31880000
💡 39.116s [test:40] node reg size: 0x80000
💡 39.122s [test:38] node reg: 0x319a0000
💡 39.128s [test:40] node reg size: 0x10000
🐛 39.134s [test:44] 

💡 39.138s [test:34] node compatible: phytium,usb2
💡 39.144s [test:38] node reg: 0x31900000
💡 39.150s [test:40] node reg size: 0x80000
💡 39.156s [test:38] node reg: 0x319b0000
💡 39.161s [test:40] node reg size: 0x10000
🐛 39.167s [test:44] 

💡 39.171s [test:34] node compatible: phytium,usb2
💡 39.178s [test:38] node reg: 0x32800000
💡 39.183s [test:40] node reg size: 0x40000
💡 39.189s [test:38] node reg: 0x32880000
💡 39.195s [test:40] node reg size: 0x40000
🐛 39.201s [test:44] 

💡 39.205s [test:34] node compatible: phytium,usb2
💡 39.211s [test:38] node reg: 0x32840000
💡 39.217s [test:40] node reg size: 0x40000
💡 39.223s [test:38] node reg: 0x328c0000
💡 39.228s [test:40] node reg size: 0x40000
🐛 39.234s [test:44] 

💡 39.238s [test:34] node compatible: phytium,dc
💡 39.245s [test:38] node reg: 0x32000000
💡 39.250s [test:40] node reg size: 0x8000
🐛 39.256s [test:44] 

💡 39.260s [test:34] node compatible: phytium,i2s
💡 39.266s [test:38] node reg: 0x32009000
💡 39.272s [test:40] node reg size: 0x1000
💡 39.278s [test:38] node reg: 0x32008000
💡 39.283s [test:40] node reg size: 0x1000
🐛 39.289s [test:44] 

💡 39.293s [test:34] node compatible: phytium,i2s
💡 39.300s [test:38] node reg: 0x3200b000
💡 39.305s [test:40] node reg size: 0x1000
💡 39.311s [test:38] node reg: 0x3200a000
💡 39.317s [test:40] node reg size: 0x1000
🐛 39.322s [test:44] 

💡 39.327s [test:34] node compatible: phytium,pmdk-dp
🐛 39.333s [test:44] 

💡 39.337s [test:34] node compatible: phytium,mbox
💡 39.344s [test:38] node reg: 0x32a00000
💡 39.349s [test:40] node reg size: 0x1000
🐛 39.355s [test:44] 

💡 39.359s [test:34] node compatible: phytium,rng
💡 39.366s [test:38] node reg: 0x32a36000
💡 39.371s [test:40] node reg size: 0x1000
🐛 39.377s [test:44] 

💡 39.381s [test:34] node compatible: phytium,pe220x-sram-ns
💡 39.389s [test:34] node compatible: mmio-sram
💡 39.395s [test:38] node reg: 0x32a10000
💡 39.400s [test:40] node reg size: 0x2000
🐛 39.406s [test:44] 

💡 39.410s [test:34] node compatible: arm,scmi-shmem
💡 39.417s [test:38] node reg: 0x32a11000
💡 39.422s [test:40] node reg size: 0x400
🐛 39.428s [test:44] 

💡 39.432s [test:34] node compatible: arm,scmi-shmem
💡 39.439s [test:38] node reg: 0x32a11400
💡 39.445s [test:40] node reg size: 0x400
🐛 39.450s [test:44] 

💡 39.454s [test:34] node compatible: phytium,hwspinlock
💡 39.461s [test:38] node reg: 0x32b36000
💡 39.467s [test:40] node reg size: 0x1000
🐛 39.473s [test:44] 

💡 39.477s [test:34] node compatible: pci-host-ecam-generic
💡 39.484s [test:38] node reg: 0x40000000
💡 39.490s [test:40] node reg size: 0x10000000
🐛 39.496s [test:44] 

💡 39.500s [test:34] node compatible: phytium,pe220x-edac
💡 39.507s [test:38] node reg: 0x32b28000
💡 39.513s [test:40] node reg size: 0x1000
💡 39.518s [test:38] node reg: 0x31400000
💡 39.524s [test:40] node reg size: 0x1000
💡 39.530s [test:38] node reg: 0x31401000
💡 39.535s [test:40] node reg size: 0x1000
🐛 39.541s [test:44] 

💡 39.545s [test:34] node compatible: phytium,hda
💡 39.552s [test:38] node reg: 0x28006000
💡 39.557s [test:40] node reg size: 0x1000
🐛 39.563s [test:44] 

💡 39.567s [test:34] node compatible: phytium,i2s
💡 39.573s [test:38] node reg: 0x28009000
💡 39.579s [test:40] node reg size: 0x1000
💡 39.585s [test:38] node reg: 0x28005000
💡 39.590s [test:40] node reg size: 0x1000
🐛 39.596s [test:44] 

💡 39.600s [test:34] node compatible: phytium,canfd
💡 39.607s [test:38] node reg: 0x2800a000
💡 39.612s [test:40] node reg size: 0x1000
🐛 39.618s [test:44] 

💡 39.622s [test:34] node compatible: phytium,canfd
💡 39.629s [test:38] node reg: 0x2800b000
💡 39.635s [test:40] node reg size: 0x1000
🐛 39.640s [test:44] 

💡 39.644s [test:34] node compatible: phytium,keypad
💡 39.651s [test:38] node reg: 0x2807a000
💡 39.657s [test:40] node reg size: 0x1000
🐛 39.662s [test:44] 

💡 39.667s [test:34] node compatible: phytium,pe220x-xhci
💡 39.674s [test:38] node reg: 0x31a08000
💡 39.679s [test:40] node reg size: 0x18000
🐛 39.685s [test:44] 

💡 39.689s [test:34] node compatible: phytium,pe220x-xhci
💡 39.696s [test:38] node reg: 0x31a28000
💡 39.702s [test:40] node reg size: 0x18000
🐛 39.708s [test:44] 

💡 39.712s [test:34] node compatible: generic-ahci
💡 39.718s [test:38] node reg: 0x31a40000
💡 39.724s [test:40] node reg size: 0x1000
🐛 39.730s [test:44] 

💡 39.734s [test:34] node compatible: generic-ahci
💡 39.740s [test:38] node reg: 0x32014000
💡 39.746s [test:40] node reg size: 0x1000
🐛 39.752s [test:44] 

💡 39.756s [test:34] node compatible: cdns,phytium-gem-1.0
💡 39.763s [test:38] node reg: 0x3200c000
💡 39.769s [test:40] node reg size: 0x2000
🐛 39.774s [test:44] 

💡 39.779s [test:34] node compatible: cdns,phytium-gem-1.0
💡 39.786s [test:38] node reg: 0x3200e000
💡 39.791s [test:40] node reg size: 0x2000
🐛 39.797s [test:44] 

💡 39.801s [test:34] node compatible: cdns,phytium-gem-1.0
💡 39.808s [test:38] node reg: 0x32010000
💡 39.814s [test:40] node reg size: 0x2000
🐛 39.820s [test:44] 

💡 39.824s [test:34] node compatible: cdns,phytium-gem-1.0
💡 39.831s [test:38] node reg: 0x32012000
💡 39.837s [test:40] node reg size: 0x2000
🐛 39.842s [test:44] 

💡 39.846s [test:34] node compatible: phytium,vpu
💡 39.853s [test:38] node reg: 0x32b00000
💡 39.858s [test:40] node reg size: 0x20000
🐛 39.864s [test:44] 

💡 39.868s [test:34] node compatible: phytium,i2c
💡 39.875s [test:38] node reg: 0x28026000
💡 39.880s [test:40] node reg size: 0x1000
🐛 39.886s [test:44] 

💡 39.890s [test:34] node compatible: dallas,ds1339
💡 39.897s [test:38] node reg: 0x68
🐛 39.902s [test:44] 

💡 39.906s [test:34] node compatible: phytium,i2c
💡 39.912s [test:38] node reg: 0x28030000
💡 39.918s [test:40] node reg size: 0x1000
🐛 39.924s [test:44] 

💡 39.928s [test:34] node compatible: everest,es8336
💡 39.935s [test:38] node reg: 0x10
🐛 39.940s [test:44] 

💡 39.944s [test:34] node compatible: arm,pl011
💡 39.950s [test:34] node compatible: arm,primecell
💡 39.956s [test:38] node reg: 0x28014000
💡 39.962s [test:40] node reg size: 0x1000
🐛 39.968s [test:44] 

💡 39.972s [test:34] node compatible: phytium,i2c
💡 39.978s [test:38] node reg: 0x28016000
💡 39.984s [test:40] node reg size: 0x1000
🐛 39.990s [test:44] 

💡 39.994s [test:34] node compatible: phytium,i2c
💡 40.000s [test:38] node reg: 0x28024000
💡 40.006s [test:40] node reg size: 0x1000
🐛 40.012s [test:44] 

💡 40.016s [test:34] node compatible: arm,pl011
💡 40.022s [test:34] node compatible: arm,primecell
💡 40.028s [test:38] node reg: 0x2802a000
💡 40.034s [test:40] node reg size: 0x1000
🐛 40.040s [test:44] 

💡 40.044s [test:34] node compatible: arm,pl011
💡 40.050s [test:34] node compatible: arm,primecell
💡 40.057s [test:38] node reg: 0x28032000
💡 40.062s [test:40] node reg size: 0x1000
🐛 40.068s [test:44] 

🐛 40.072s [test:44]

💡 40.076s [test:38] node reg: 0x80000000
💡 40.082s [test:40] node reg size: 0x7c000000
🐛 40.088s [test:44] 

💡 40.092s [test:34] node compatible: gpio-leds
🐛 40.098s [test:44] 

🐛 40.103s [test:44]

💡 40.107s [test:34] node compatible: simple-audio-card
🐛 40.114s [test:44] 

🐛 40.118s [test:44]

🐛 40.122s [test:44] 

🐛 40.127s [test:44]
