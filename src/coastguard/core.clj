(ns coastguard.core
  (:gen-class)
  (:require
   [clojure.string :as str]
   [clojure.tools.cli :as cli]
   [coastguard.dns :refer [hostname-exists?]]
   [coastguard.homoglyphs :refer [flatten-endpoint-mutations mutate]]))

(defn cli-options
  "Commandline options"
  []
  (let [opts [["-d" "--debug" "Enable debug mode"]]]
    (cli/parse-opts *command-line-args* opts)))

(defn -main
  "Running that program!"
  [& args]
  (let [{:keys [options arguments errors summary]} (cli-options)]
    (if (seq errors)
      (println (str "Error: " errors))
      (do
        (when (:debug options) (println "Debug mode is on"))
        (when (seq arguments)
          (let [fqdn (first arguments)]
            (println fqdn)))))))

(defn deconstruct-endpoint [endpoint]
  (str/split endpoint #"\."))

(defn check-homoglyphs [endpoint]
  (let [deconstructed-endpoint (deconstruct-endpoint endpoint)
        mutations (flatten-endpoint-mutations (map mutate (butlast deconstructed-endpoint)) (last deconstructed-endpoint))]
    (map hostname-exists? mutations)))

(defn get-valid-endpoints [checked-endpoints]
  (filter (fn [x]
            (true? (last x))) checked-endpoints))