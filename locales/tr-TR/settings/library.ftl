settings-library-watch-section-desc = skwd-wall dışında eklenen dosyaları algılayın ve kitaplığı güncel tutun.
settings-library-watch-fallback-label = Düzenli tarama yedeği
settings-library-watch-fallback-desc = Yalnızca yerel dosya izleyicisinin izleyemediği kitaplık klasörlerini kontrol edin. Değişiklikleri kaçıran ağ veya FUSE bağlamaları için etkinleştirin, ardından skwd-walld'yi yeniden başlatın.
settings-library-watch-interval-label = Tarama aralığı
settings-library-watch-interval-desc = Sınırlı kontroller arasında beklenecek saniye sayısı. Düşük değerler değişiklikleri daha erken bulur ancak dosya sistemini daha sık okur. Değiştirdikten sonra skwd-walld'yi yeniden başlatın.
settings-library-watch-unknown-label = İzleyici durumu alınamıyor
settings-library-watch-unknown-desc = Bu arka plan hizmeti kitaplık izleme durumunu bildirmiyor. skwd-walld'yi güncelleyin veya yeniden başlatın.
settings-library-watch-poll-failed-label = Tarama bir kitaplık klasörünü okuyamıyor
settings-library-watch-poll-failed-desc = Yapılandırılan tüm kitaplık klasörlerinin bağlı ve okunabilir olduğunu kontrol edin. Tarama { $interval } saniye sonra yeniden denenecek.
settings-library-watch-polling-label = Düzenli tarama yedeği etkin
settings-library-watch-polling-desc = Yerel izleme { $count ->
    [one] bir kitaplık klasörü
   *[other] { $count } kitaplık klasörü
    } için başarısız oldu. Her { $interval } saniyede en fazla { $budget } öğe kontrol edilir. Son başarılı eşitleme: { $convergence }.
settings-library-watch-recovering-label = Yerel izleme yeniden çalışıyor
settings-library-watch-recovering-desc = Yerel izleyici yeniden etkin. Kitaplığın güncel olduğu doğrulanmadan önce tam devir taraması sürüyor.
settings-library-watch-unavailable-label = Kitaplık izleme kullanılamıyor
settings-library-watch-unavailable-desc = Yerel dosya izleme başarısız oldu ve düzenli tarama yedeği kapalı. Düzenli tarama yedeğini etkinleştirin, ardından skwd-walld'yi yeniden başlatın.
settings-library-watch-recovered-label = Yerel izleme geri yüklendi
settings-library-watch-recovered-desc = Yerel izleyici ve devir taraması güncel. Son başarılı eşitleme: { $convergence }.
settings-library-watch-native-label = Yerel dosya izleme
settings-library-watch-native-desc = Tüm kitaplık klasörlerinde dosya sistemi olayları etkin. Düzenli tarama beklemede.
settings-library-watch-convergence-never = Henüz tamamlanmadı
settings-library-watch-convergence-seconds = { $value ->
    [one] 1 saniye önce
   *[other] { $value } saniye önce
    }
settings-library-watch-convergence-minutes = { $value ->
    [one] 1 dakika önce
   *[other] { $value } dakika önce
    }
settings-library-watch-convergence-hours = { $value ->
    [one] 1 saat önce
   *[other] { $value } saat önce
    }
