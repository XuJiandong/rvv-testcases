#!/usr/bin/python3

import os


def lmul_to_str(lmul):
    if lmul == -8:
        return 'mf8'
    elif lmul == -4:
        return 'mf4'
    elif lmul == -2:
        return 'mf2'
    elif lmul == 1:
        return 'm1'
    elif lmul == 2:
        return 'm2'
    elif lmul == 4:
        return 'm4'
    elif lmul == 8:
        return 'm8'


def get_sp(v):
    if v == 0:
        return ''
    elif v == 1:
        return '  '
    elif v == 2:
        return '    '
    elif v == 3:
        return '      '
    elif v == 4:
        return '        '
    elif v == 5:
        return '          '
    elif v == 6:
        return '            '
    else:
        assert(False)


def gen_sew_lmul_case(sew, lmul, avl, sp_v):
    print(get_sp(sp_v) + str(sew) + ' => {')
    if avl == -1:
        print(get_sp(sp_v + 1) + 'rvv_asm!("mv t1, {0}", "vsetvli t0, t1, e' + str(
            sew) + ', ' + lmul_to_str(lmul) + '", "mv {1}, t0", in (reg) avl, out (reg) vl);')
        print(get_sp(sp_v + 1) + 'vl')
    else:
        print(get_sp(sp_v + 1) + 'rvv_asm!("vsetivli t0, ' + str(avl) + ', e' +
              str(sew) + ', ' + lmul_to_str(lmul) + '", "mv {0}, t0", out (reg) vl);')
        print(get_sp(sp_v + 1) + 'vl')
    print(get_sp(sp_v) + '}')


G_LMUL = [-8, -4, -2, 1, 2, 4, 8]
G_SEW = [8, 16, 32, 64, 128, 256, 512, 1024]


def gen_lmul_case(lmul, avl, sp_v):
    print(get_sp(sp_v) + str(lmul) + '=> match sew {')
    for sew in G_SEW:
        gen_sew_lmul_case(sew, lmul, avl, sp_v + 2)
    print(get_sp(sp_v + 1) + '_ => panic!("Abort")')
    print(get_sp(sp_v) + '}')


def gen_vsetvli():
    print('fn vsetvli(avl: u64, sew: u64, lmul: i64) -> u64 {')
    print('  unsafe {')
    print('  let mut vl: u64;')
    print('    match lmul {')
    for lmul in G_LMUL:
        gen_lmul_case(lmul, -1, 3)
    print('      _ => panic!("Abort")')
    print('    }')
    print('  }')
    print('}')


def gen_vsetivli():
    sp_v = 0
    print('fn vsetivli(avl: u64, sew: u64, lmul: i64) -> u64 {')
    print('  unsafe {')
    print('    let mut vl: u64;')
    print('    match avl {')

    for vl in range(0, 32):
        print(get_sp(sp_v + 3) + str(vl) + ' => {')
        print(get_sp(sp_v + 4) + 'match lmul {')
        for lmul in G_LMUL:
            gen_lmul_case(lmul, vl, sp_v + 3)

        print(get_sp(sp_v + 3) + '_ => panic!("Abort")')
        print(get_sp(sp_v + 4) + '}')
        print(get_sp(sp_v + 3) + '}')

    print(get_sp(sp_v + 3) + '_ => panic!("Abort")')
    print('    }')
    print('  }')
    print('}')


os.system('clear')

gen_vsetvli()
gen_vsetivli()
