(ns coastguard.tlds
  (:require
   [clojure.java.io :refer [reader]]))

(defn read-tlds-txt [path]
  (with-open [rdr (reader path)]
    (doall (line-seq rdr))))

(defn mutate-using-tlds []
  (list (read-tlds-txt "./tlds.txt")))