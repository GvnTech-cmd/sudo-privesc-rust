# Tehdit Modeli: Sudo + `find` Üzerinden Yetki Yükseltme (Araştırma Projesi)

Bu belge, **siber güvenlik dersinde işlenen STRIDE** çerçevesini kullanarak `sudo-privesc-rust` projesinin tehdit modelini özetler. Amaç, sistemin güvenlik açısından nerede durduğunu ve saldırganın neyi hedeflediğini net göstermektir.

---

## STRIDE Analizi (Özet Tablo)

STRIDE, Microsoft’un geliştirdiği ve tehditleri altı kategoride toplayan bir yöntemdir. Bu projede ilgili olanları basitçe eşleştiriyorum:

| Tehdit (STRIDE) | Kısa açıklama | Bu projede / örnek senaryo |
|-----------------|---------------|----------------------------|
| **Spoofing (Kimliğe bürünme)** | Başkasının kimliğiyle hareket etmek | Normal kullanıcı zaten kendi hesabıyla oturum açmıştır; saldırı “başka bir kullanıcıyı taklit etmek”ten çok, **sudo ile root kimliğine geçmek** ile ilgilidir. Doğrudan ana tehdit değil. |
| **Tampering (Değiştirme)** | Veriyi veya yapılandırmayı izinsiz değiştirmek | Kötü yapılandırılmış `/etc/sudoers` kuralları **sistem yöneticisi tarafından** yanlışlıkla yazılmış olabilir; saldırgan bunu değiştirmeden **mevcut kuralı kötüye kullanır**. Tehdit, yapılandırma hatasının *sonucu* olarak ortaya çıkar. |
| **Repudiation (İnkar)** | Yapılan işi sonradan inkâr edememe / kayıt eksikliği | Bu araç tek başına denetim günlüğü üretmez. Gerçek sistemlerde `sudo` ve auth logları inkârı zorlaştırır; proje o düzeyde ele alınmamıştır. |
| **Information Disclosure (Bilgi sızdırma)** | Hassas bilginin sızmaması gerekirken sızmış olması | `sudo -l` çıktısı, yetkiler hakkında bilgi verir; saldırgan zaten yetkili kullanıcıysa bu “bilgi”yi okuyabilir. Asıl risk, bu bilgiyle **sonraki adımı** planlamaktır. |
| **Denial of Service (Hizmet dışı bırakma)** | Sistemi veya hizmeti kullanılamaz hale getirmek | `find` veya shell ile sistem bozulabilir; bu projenin odak noktası **kök shell almak** olduğu için DoS ikincil bir ihtimaldir. |
| **Elevation of Privilege (Yetki yükseltme)** | Düşük yetkiden yüksek yetkiye (ör. root) çıkmak | **Bu projenin merkezindeki tehdit budur.** Kullanıcı, `sudo` üzerinden `find`’e verilen aşırı yetkiyi kullanarak **root** olmaya çalışır. |

**Özet:** STRIDE içinde bu senaryoyu en iyi karşılayan başlık **Elevation of Privilege**’dir. Diğer başlıklar bağlam ve yan etkiler için not düşmek içindir.

---

## Saldırı Yüzeyi (Attack Surface)

Kısaca “saldırgan nereden başlar, nereye gitmek ister?”

| Kavram | Bu projede anlamı |
|--------|-------------------|
| **Giriş noktası (User Shell)** | Saldırganın (veya test senaryosundaki kullanıcının) zaten **normal kullanıcı hesabı** ile komut satırına erişimi vardır. Yani “dışarıdan sıfırdan ağ saldırısı” değil; **yerel kullanıcı oturumu** başlangıç noktasıdır. |
| **Hedef (Root)** | Amaç, işletim sisteminde en yüksek yetkilerden biri olan **root** (süper kullanıcı) ile komut çalıştırmaktır. Böylece dosya silme, kullanıcı ekleme, yapılandırma değiştirme gibi kritik işlemler mümkün hale gelir. |
| **Köprü** | `sudo`, normal kullanıcıya belirli komutları root olarak çalıştırma izni verir. Bu projede **hatalı izin**: `find` komutuna **NOPASSWD** (parolasız) yetki verilmesi, bu köprüyü kötüye kullanmayı kolaylaştırır. |

Yani yüzey: **yerel kullanıcı + hatalı sudo kuralı → root yetkisi**.

---

## `find` Zafiyeti Neden “Elevation of Privilege” (Yetki Yükseltme)?

**1. sınıf düzeyinde teknik açıklama:**

- **Elevation of Privilege**, “şu an sahip olduğum yetkiden daha yüksek bir yetkiye çıkmak” demektir.
- Normal kullanıcı dosyaları okuyup yazabilir ama **sistem genelinde** root gibi davranamaz.
- `sudo` ile `find`’e izin verildiğinde, `find` komutunun **`-exec`** özelliği sayesinde root olarak **başka bir program** (örneğin shell) çalıştırılabilir.
- Sonuç: Aynı kişi önce “sıradan kullanıcı”, birkaç komut sonra **root yetkisiyle shell**. Yetki seviyesi yükselmiş olur; bu yüzden STRIDE’da bu kategoriye girer.
- Burada “kimlik çalma” veya “mesajı değiştirme” değil, **yetki seviyesinin artması** ana olaydır.

Kısa cümle: **Düşük yetkili kullanıcı, hatalı sudo kuralını kullanarak root olur → bu da tipik bir yetki yükseltme (EoP) örneğidir.**

---

## Kapsam Notu (Öğrenci Projesi)

Bu tehdit modeli, gerçek üretim sistemlerindeki tüm saldırıları kapsamaz; özellikle **ağ üzerinden ilk erişim**, **zafiyetli web uygulaması** veya **fiziksel erişim** gibi konular bu belgenin dışındadır. Odak: **yerel kullanıcıdan root’a çıkış** ve STRIDE ile bunun sınıflandırılmasıdır.
