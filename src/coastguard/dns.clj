(ns coastguard.dns)

(defn hostname-exists? [hostname]
  (try
    (let [_ (java.net.InetAddress/getByName hostname)]
      (seq [hostname true]))
    (catch java.net.UnknownHostException _
      (seq [hostname false]))))