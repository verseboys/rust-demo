 Os { code: 5, kind: PermissionDenied, message: "拒绝访问。

 cargo run --release

您好,我是独立顾问(Independent Advisor)Dexter,请让我来帮助您.

帮您查了下Windows10 2004版本中锁屏背景在注册表中相关的设置分别位于:

HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\Personalization

HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\Policies\Microsoft\Windows\Personalization

设置项名称均为:

LockScreenImage

可以分别修改上面两处位置的LockScreenImage内容指向要使用的图片完整路径,修改完成后需要重新启动一下系统生效.

希望以上信息可以帮得到您,如果还有什么疑问的话,欢迎继续提问.

.\lockscreen_wallpaper.exe set 'C:\Users\project\wallpaper.jpg'

.\lockscreen_wallpaper.exe remove

[官方设置教程-关于使用服务组策略统一域内 Windows 专业版本 成员锁屏桌面问题](https://answers.microsoft.com/zh-hans/windows/forum/all/%E5%85%B3%E4%BA%8E%E4%BD%BF%E7%94%A8%E6%9C%8D/45f46642-d893-4a29-914b-56423bafcf03#:~:text=%E6%A0%B9%E6%8D%AE%E6%9C%8D%E5%8A%A1%E7%AB%AF%E7%BB%84)
