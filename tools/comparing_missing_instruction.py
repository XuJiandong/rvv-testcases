

f1_path = 'data/1.txt'
f2_path = 'data/2.txt'

def get_f1():
    ff = open(f1_path, mode='r', encoding='utf8')
    f1_data = ff.read()
    ff.close()
    f1_data = f1_data.splitlines()
    ds = []
    for data in f1_data:
        if len(data) == 0:
            continue

        if data.find('[x] [x]') == -1:
            continue
        
        data = data.split(' ')
        ds.append(data[2])
    return ds

def get_f2():
    ff = open(f2_path, mode='r', encoding='utf8')
    f2_data = ff.read()
    ff.close()
    f2_data = f2_data.splitlines()
    ds = []
    for data in f2_data:
        if len(data) == 0:
            continue
        data = data.split(',')
        ds.append(data[0])
    return ds

d1 = get_f1()
d2 = get_f2()


ret = []
for d in d1:
    flag = False
    for dd in d2:
        if d == dd:
            flag = True
            break
    if flag == False:
        ret.append(d)

for d in ret:
    print(d)

