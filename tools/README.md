# README


## Python tools, for repetitive tasks

* Python using 3.x version

### generate_expand_bit.py
Replacing macros in code that can automatically generate test cases for other bits.

```Rust
match sew {
  // @RVV_CASES_BEGIN
  64 => { let a: u64 = 0; }
  // @RVV_CASES_END
  // @RVV_CASES_BEGIN
  256 => { let a = E256::from(0u32); }
  // @RVV_CASES_END
}
```
to
``` Rust
match sew {
  8 => { let a: u8 = 0; }
  16 => { let a: u16 = 0; }
  32 => { let a: u32 = 0; }
  64 => { let a: u64 = 0; }

  128 => { let a = E128::from(0u32); }
  256 => { let a = E256::from(0u32); }
  512 => { let a = E512::from(0u32); }
  1024 => { let a = E1024::from(0u32);}
}
```
* will replace the value of the corresponding number of digits.
* Only supports autogen from 64 and 256.
* Post-generated code may need to be reformatted.


### generate_vi.py
The immediate series instruction set requires rvv_asm code to be compiled before it can be used, so many repeated scripts are required in many places, so it is used to generate code.

Example:
```shell
generate_vsetvl.py "vand.vi v24, v8, %imm%" -16 16
```

### generate_vsetvl.py
Test code for generating vsetvl series
``` cases/src/intrinsic_setvl.rs ```



### comparing_missing_instruction.py
Can find out which instruction has not been tested.

1. Comment out the code about log in ```cases/src/misc.rs:69 test_case```
2. In ```cases/src/runner.rs run_template_ext``` output desc value
3. Copy output to tools/data/2.txt
4. Copy implemented list to tools/data/1.txt

Example:
1.txt
```
[x] [x] vadd.vv        31..26=0x00 vm vs2 vs1 14..12=0x0 vd 6..0=0x57
[x] [x] vadd.vx        31..26=0x00 vm vs2 rs1 14..12=0x4 vd 6..0=0x57
[x] [x] vadd.vi        31..26=0x00 vm vs2 simm5 14..12=0x3 vd 6..0=0x57
[x] [x] vsub.vv         31..26=0x02 vm vs2 vs1 14..12=0x0 vd 6..0=0x57
[x] [x] vsub.vx        31..26=0x02 vm vs2 rs1 14..12=0x4 vd 6..0=0x57
[x] [x] vrsub.vx       31..26=0x03 vm vs2 rs1 14..12=0x4 vd 6..0=0x57
[x] [x] vrsub.vi       31..26=0x03 vm vs2 simm5 14..12=0x3 vd 6..0=0x57

[x] [x] vwaddu.vv      31..26=0x30 vm vs2 vs1 14..12=0x2 vd 6..0=0x57
[x] [x] vwaddu.vx      31..26=0x30 vm vs2 rs1 14..12=0x6 vd 6..0=0x57
[x] [x] vwsubu.vv      31..26=0x32 vm vs2 vs1 14..12=0x2 vd 6..0=0x57
[x] [x] vwsubu.vx      31..26=0x32 vm vs2 rs1 14..12=0x6 vd 6..0=0x57
```

2.txt
```
vwaddu.vv
vwaddu.vx
vwsubu.vv
vwsubu.vx
vadd.vv
vadd.vx
vadd.vi
vsub.vx
vrsub.vx
vrsub.vi
```

output
```
vsub.vv
```