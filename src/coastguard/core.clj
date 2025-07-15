(ns coastguard.core
  (:gen-class)
  (:require [clojure.tools.cli :as cli]))

(def cli-options
  (let [opts [["-d" "--debug" "Enable debug mode"]]]
    (cli/parse-opts *command-line-args* opts)))

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (let [{:keys [options arguments errors summary]} (cli-options)]
    (if (seq errors)
      (println (str "Error: " errors))
      (do
        (when (:debug options) (println "Debug mode is on"))
        (when (:levenshtein-distance options)
          (println (str "Levenshtein distance set to " (:levenshtein-distance options))))
        (when (seq arguments)
          (let [fqdn (first arguments)]
            (println fqdn)))))))
