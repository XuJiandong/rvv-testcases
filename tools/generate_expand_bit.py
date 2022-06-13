#!/usr/bin/python3


import os


def GetSrcData(src_path):
    try:
        f = open(src_path, 'r', encoding='utf8')
        data = f.read()
        f.close()
        return data
    except Exception as _:
        return None


g_begin_mark = '// @RVV_CASES_BEGIN'
g_end_mark = '// @RVV_CASES_END'


def GetLineNum(data, mark_pos):
    count = 1
    pos = 0
    while True:
        i = data.find('\n', pos)
        if i == -1:
            break
        pos = i + 1
        if i >= mark_pos:
            break

        count += 1
    return count


def IsValidChar(c):
    return c != ' ' and c != '\n' and c != '\r'


def GetValidStr(data):
    i = 0
    for c in data:
        if IsValidChar(c) == True:
            break
        i += 1
    data = data[i:]

    i = len(data)
    for c in reversed(data):
        if IsValidChar(c) == True:
            break
        i -= 1
    data = data[:i]
    return data


def GetSew(data):
    i = data.find('=>')
    if i == -1:
        return None
    return int(GetValidStr(data[:i]))


def Expand(data):
    sew = GetSew(data)

    ret = ''
    if sew == 64:
        rep_sew = [8, 16, 32, 64]
    elif sew == 256:
        rep_sew = [128, 256, 512, 1024]
    else:
        print('error, sew: ' + str(sew))
        return None

    d1 = data.replace(str(sew), '{@sew}')
    d1 = d1.replace(str(int(sew / 2)), '{@sew/2}')
    d1 = d1.replace(str(sew * 2), '{@sew*2}')
    d1 = d1.replace(str(sew - 1), '{@sew-1}')

    for s in rep_sew:
        if s == sew:
            d2 = data
        else:
            d2 = d1.replace('{@sew}', str(s))
            d2 = d2.replace('{@sew/2}', str(int(s / 2)))
            d2 = d2.replace('{@sew*2}', str(s * 2))
            d2 = d2.replace('{@sew-1}', str(s - 1))
        ret += d2
    return ret


def ExpandMacor(src_path):
    data = GetSrcData(src_path)
    if data == None:
        return

    need_rewrite = False
    pos = 0
    while True:
        i1 = data.find(g_begin_mark, pos)
        if i1 == -1:
            break
        i2 = data.find(g_end_mark, i1 + len(g_begin_mark))
        if i2 == -1:
            pos = i1 + len(g_begin_mark)
            print(src_path + ":" + str(GetLineNum(data, i1)) +
                  "  path mark has not end")
            continue

        macro_data = data[i1 + len(g_begin_mark): i2]
        macro_data2 = Expand(macro_data)
        if macro_data2 == None:
            print("error in " + src_path + ":" + str(GetLineNum(data, i1)))
            assert(True)
        if macro_data != macro_data2:
            data = data[: i1] + macro_data2 + data[i2 + len(g_end_mark):]
            need_rewrite = True

    if need_rewrite == True:
        data = data.replace('}\n        \n ', '}\n ')
        data = data.replace('}\n        \n        \n ', '}\n ')
        try:
            f = open(src_path, 'w', encoding='utf8')
            f.write(data)
            f.close()
        except Exception as _:
            pass
    return


path = "/home/joii/code/rvv-testcases/cases/src"

for root, dirs, files in os.walk(path):
    for file in files:
        src_path = os.path.join(root, file)
        ExpandMacor(src_path)
