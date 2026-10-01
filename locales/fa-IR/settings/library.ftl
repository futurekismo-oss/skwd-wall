settings-library-watch-section-desc = فایل‌هایی را که بیرون از skwd-wall افزوده می‌شوند شناسایی کنید تا کتابخانه به‌روز بماند.
settings-library-watch-fallback-label = پشتیبان بررسی دوره‌ای
settings-library-watch-fallback-desc = فقط پوشه‌های کتابخانه‌ای را بررسی کنید که پایش بومی فایل‌ها نمی‌تواند آن‌ها را بپاید. این گزینه را برای اتصال‌های شبکه‌ای یا FUSE که تغییرات را از دست می‌دهند فعال کنید، سپس skwd-walld را دوباره راه‌اندازی کنید.
settings-library-watch-interval-label = فاصلهٔ بررسی دوره‌ای
settings-library-watch-interval-desc = فاصلهٔ بین بررسی‌های محدود، بر حسب ثانیه. مقدار کمتر تغییرات را زودتر پیدا می‌کند، اما بیشتر از سامانهٔ فایل می‌خواند. پس از تغییر، skwd-walld را دوباره راه‌اندازی کنید.
settings-library-watch-unknown-label = وضعیت پایشگر در دسترس نیست
settings-library-watch-unknown-desc = این دیمن وضعیت پایش کتابخانه را گزارش نمی‌کند. skwd-walld را به‌روزرسانی یا دوباره راه‌اندازی کنید.
settings-library-watch-poll-failed-label = بررسی دوره‌ای نمی‌تواند پوشهٔ کتابخانه را بخواند
settings-library-watch-poll-failed-desc = مطمئن شوید همهٔ پوشه‌های کتابخانهٔ تنظیم‌شده متصل و خواندنی‌اند. بررسی دوره‌ای پس از { $interval } ثانیه دوباره تلاش می‌کند.
settings-library-watch-polling-label = پشتیبان بررسی دوره‌ای فعال است
settings-library-watch-polling-desc = پایش بومی برای { $count ->
    [one] { $count } پوشهٔ کتابخانه
   *[other] { $count } پوشهٔ کتابخانه
    } ناموفق بود. هر { $interval } ثانیه حداکثر { $budget } ورودی بررسی می‌شود. آخرین همگام‌سازی موفق: { $convergence }.
settings-library-watch-recovering-label = پایش بومی بازیابی شد
settings-library-watch-recovering-desc = پایشگر بومی دوباره فعال است. پیش از آنکه کتابخانه به‌روز اعلام شود، یک پویش کامل انتقالی همچنان در حال اجرا است.
settings-library-watch-unavailable-label = پایش کتابخانه در دسترس نیست
settings-library-watch-unavailable-desc = پایش بومی فایل‌ها ناموفق بود و پشتیبان بررسی دوره‌ای خاموش است. آن را فعال و سپس skwd-walld را دوباره راه‌اندازی کنید.
settings-library-watch-recovered-label = پایش بومی بازگردانده شد
settings-library-watch-recovered-desc = پایشگر بومی و پویش انتقالی آن به‌روز هستند. آخرین همگام‌سازی موفق: { $convergence }.
settings-library-watch-native-label = پایش بومی فایل‌ها
settings-library-watch-native-desc = رویدادهای سامانهٔ فایل برای همهٔ پوشه‌های کتابخانه فعال‌اند. بررسی دوره‌ای غیرفعال است.
settings-library-watch-convergence-never = هنوز کامل نشده
settings-library-watch-convergence-seconds = { $value ->
    [one] { $value } ثانیه پیش
   *[other] { $value } ثانیه پیش
    }
settings-library-watch-convergence-minutes = { $value ->
    [one] { $value } دقیقه پیش
   *[other] { $value } دقیقه پیش
    }
settings-library-watch-convergence-hours = { $value ->
    [one] { $value } ساعت پیش
   *[other] { $value } ساعت پیش
    }
