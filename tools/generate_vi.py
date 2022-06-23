#!/usr/bin/python3

import sys
import os

def main():
    os.system("clear")
    vi_template = sys.argv[1]
    imm_begin = int(sys.argv[2])
    imm_end = int(sys.argv[3])

    print('fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {')
    print('  let imm = i64::from_le_bytes(rhs.try_into().unwrap());')
    print('  unsafe {')
    print('  match imm {')

    i = imm_begin
    while True:
        data = vi_template
        data = data.replace('%imm%', str(i))

        print('    ' + str(i) + ' => {')
        print('      match mask_type {')
        print('        MaskType::Enable => { rvv_asm!(\"' + data + ', v0.t\"); }')
        print('        MaskType::Disable => { rvv_asm!(\"' + data + '\"); }')
        print('        _ => panic!("Abort"),')
        print('      }')
        print('    }')
        if i == imm_end:
            break

        if imm_begin < imm_end:
            i = i + 1
        else:
            i = i - 1

    print('    _ => {')
    print('      panic!("Abort");')
    print('    }')

    print('  }')
    print('  }')
    print('}')


if __name__ == '__main__':
    main()
