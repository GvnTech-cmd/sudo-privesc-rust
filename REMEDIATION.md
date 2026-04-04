# Giderim (Remediation): `sudo` + `find` (NOPASSWD) Riski

Bu belge, sistem yöneticilerinin **`/usr/bin/find` için parolasız (`NOPASSWD`) sudo izni** vermesinin yarattığı yetki yükseltme riskini **nasıl kaldıracağını** özetler.

---

## Sorun ne?

`NOPASSWD` ile `find` verildiğinde, kullanıcı genelde `-exec` ile root bağlamında rastgele komut çalıştırabilir. Bu, **en az yetki prensibine** aykırıdır.

---

## Doğru düzenleme aracı: `visudo`

`/etc/sudoers` dosyasını **doğrudan normal bir metin düzenleyiciyle** açmayın; sözdizimi hatası sistemi kilitleyebilir.

**Önerilen komut:**

```bash
sudo visudo
```

İsteğe bağlı olarak ayrı bir dosya kullanıyorsanız (dağıtıma göre):

```bash
sudo visudo -f /etc/sudoers.d/99-custom
```

`visudo`, kaydetmeden önce **sözdizimini kontrol eder**; hata varsa uyarı verir.

---

## Ne yapmalı? (Güvenli kurallar)

1. **`find` için `NOPASSWD` satırını kaldırın veya daraltın**  
   - Genel ifade: `ALL ALL=(ALL) NOPASSWD: /usr/bin/find` gibi geniş kurallar **kullanmayın**.

2. **Gerçekten `find` şart değilse**  
   - İhtiyaç duyulan işi yapan **daha dar kapsamlı** bir betik veya araç tanımlayın; sadece o betiğe sınırlı izin verin (mümkünse **parola ile** veya çok net argüman kısıtları ile — `sudoers` içinde argüman kısıtlama sürüme göre değişir; yönetim kılavuzunu kontrol edin).

3. **En güvenlisi**  
   - Normal kullanıcılara **`find` üzerinden root yetkisi** vermeyin. Bakım işlerini **root veya ayrılmış bir yönetim hesabı** ile zamanlanmış görev veya belgelenmiş prosedürle yapın.

**Örnek yön (kavramsal):**  
Eski tehlikeli satırı silin veya yorum satırı yapın (`#` ile); yerine yalnızca gerçekten gerekli ve **mümkün olduğunca dar** kurallar ekleyin. Tam satır, kurum politikanıza ve kullanıcı gruplarına göre değişir; burada tek bir “herkese uyan” satır yoktur — önemli olan **`find`’i NOPASSWD ile serbest bırakmamaktır**.

---

## En az yetki prensibi (Least Privilege) — kısa özet

**Fikir:** Her kullanıcı ve her süreç, işini yapmak için **ihtiyaç duyduğu minimum yetkiye** sahip olmalı; fazlasını vermeyin.

- **Neden:** Yetki ne kadar genişse, hata veya kötüye kullanım o kadar büyük zarar verir.
- **Pratikte:** `sudo` kurallarında **varsayılanı “izin yok”** kabul edin; izinleri **tek tek, gerekçesiyle** ekleyin. Parolasız (`NOPASSWD`) erişimi **çok istisnai** tutun ve genel amaçlı araçları (`find`, `vim`, `python` vb.) bu kapsamdan **özellikle uzak tutun**.

---

## Doğrulama

Değişiklikten sonra:

```bash
sudo visudo -c
```

Ardından ilgili kullanıcı ile:

```bash
sudo -l
```

çıktısında **`find` için NOPASSWD kalmadığını** kontrol edin.

---

## Not

Bu belge genel güvenlik tavsiyesidir; üretim ortamında değişiklik yapmadan önce **yedek**, **değişiklik penceresi** ve kurum **değişiklik yönetimi** kurallarına uyun.
