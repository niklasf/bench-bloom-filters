'''
Plots false positive rate rates of various Rust Bloom filters.
The false pos rates are calculated in Rust in main.rs.

```
cargo run --release
python falsepos.py
```
'''

from math import log10, log2, log
import matplotlib.pyplot as plt
import csv
from matplotlib import colormaps
from matplotlib.ticker import ScalarFormatter
from matplotlib.ticker import FuncFormatter
import math
plt.rcParams['font.size'] = 18
plt.rcParams['figure.figsize'] = (16, 10)

viridis = colormaps['viridis']
magma = colormaps['magma']

alpha = 1
lw = 2.0

cm = [colormaps['Dark2'](i / 8) for i in range(8)]
filters = [
    ('fastbloom-xxhash', cm[0], 1, lw),
    #('fjall-orig', cm[1], alpha, lw),
    ('fjall', cm[2], alpha, lw),
]

fig, ax = plt.subplots()

def custom_format(yy, _):
    if yy >= 1:
        return f"{int(yy)}"
    else:
        return f"{yy:.8f}".rstrip("0").rstrip(".")

for i, (name, color, aa, lw) in enumerate(filters):
    file_name = 'Acc/%s.csv' % name
    print(file_name)
    with open(file_name, 'r') as csvfile:
        data = []
        rows = csv.reader(csvfile, delimiter = ',')
        for row in rows:
            if row[1] == 'NaN':
                continue
            num_items = float(row[0])
            avg_y = float(row[1])*100.0
            min_y = float(row[2])*100.0
            max_y = float(row[3])*100.0

            data.append((num_items, avg_y, min_y, max_y))

        x,avg_y,min_y,max_y = zip(*data)

        ax.plot(x, avg_y, color=color, label=name, linewidth=lw, alpha=aa)
        ax.fill_between(x, max_y, min_y, color = color, alpha = aa*0.15)
        ax.set_yscale('log')
        #ax.set_xscale('log')

plt.xlabel('Items Per Bit')
plt.ylabel('False Positive %')

plt.title('Bloom Filter False Positives (Lower is Better)')

plt.xlim(left=0)
plt.xlim(right=0.125)
plt.ylim(bottom=0.00000001)
plt.ylim(top=100)

'''
# Size comparison

plt.xlim(left=min(x), right=max(x))
plt.ylim(bottom=0.01)
plt.gca().yaxis.set_major_formatter(ScalarFormatter())
plt.gca().yaxis.get_major_formatter().set_scientific(False)
plt.gca().yaxis.set_major_formatter(FuncFormatter(custom_format))
'''

# Crate Comparison
plt.gca().yaxis.set_major_formatter(ScalarFormatter())
plt.gca().yaxis.get_major_formatter().set_scientific(False)
plt.gca().yaxis.set_major_formatter(FuncFormatter(custom_format))

handles,labels = ax.get_legend_handles_labels()

handles = [handles.pop()] + handles
labels = [labels.pop()] + labels

plt.grid()
# https://stackoverflow.com/questions/67033128/matplotlib-order-of-legend-entries
plt.legend(handles,labels,loc='lower right')
plt.show()

plt.savefig('falsepos.png')
