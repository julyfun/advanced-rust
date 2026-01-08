#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/fs.h>        // 包含 file_operations
#include <linux/miscdevice.h> // 包含 miscdevice
#include <linux/uaccess.h>   // 包含 copy_from_user, copy_to_user

#define HELLO_MAGIC 'x'
#define HELLO_CMD_1 _IOR(HELLO_MAGIC, 1, int) // 定义一个“读”命令
#define HELLO_CMD_2 _IOW(HELLO_MAGIC, 2, int) // 定义一个“写”命令

static long hello_ioctl(struct file *filp, unsigned int cmd, unsigned long arg)
{
    int data_from_user;
    int data_to_user = 100;

    switch (cmd) {
    case HELLO_CMD_1:
        // 将数据从内核拷贝到用户空间
        if (copy_to_user((int __user *)arg, &data_to_user, sizeof(int)))
            return -EFAULT;
        printk(KERN_INFO "Hello: CMD_1 executed, sent %d to user.\n", data_to_user);
        break;

    case HELLO_CMD_2:
        // 将数据从用户空间拷贝到内核
        if (copy_from_user(&data_from_user, (int __user *)arg, sizeof(int)))
            return -EFAULT;
        printk(KERN_INFO "Hello: CMD_2 executed, received %d from user.\n", data_from_user);
        break;

    default:
        return -ENOTTY; // 未知命令
    }
    return 0; // 成功
}

static struct file_operations hello_fops = {
    .owner          = THIS_MODULE,
    .unlocked_ioctl = hello_ioctl, // 指定ioctl处理函数
    // 还可以实现 .read, .write, .open, .release 等
};

static struct miscdevice hello_misc_device = {
    .minor = MISC_DYNAMIC_MINOR, // 动态分配次设备号
    .name = "hello",             // 设备文件将出现在 /dev/hello
    .fops = &hello_fops,         // 关联文件操作
};

static int __init hello_init(void)
{
    int ret = misc_register(&hello_misc_device);
    if (ret) {
        printk(KERN_ERR "Hello: Failed to register misc device.\n");
        return ret;
    }
    printk(KERN_INFO "Hello: Module loaded, device created at /dev/hello\n");
    return 0;
}

static void __exit hello_exit(void)
{
    misc_deregister(&hello_misc_device);
    printk(KERN_INFO "Hello: Module unloaded.\n");
}

module_init(hello_init);
module_exit(hello_exit);
MODULE_LICENSE("GPL");
