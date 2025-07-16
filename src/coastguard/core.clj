(ns coastguard.core
  (:gen-class)
  (:require
   [clojure.string :as str]
   [clojure.tools.cli :as cli]
   [coastguard.dns :refer [hostname-exists?]]
   [coastguard.homoglyphs :refer [mutate-using-homoglyphs]]
   [coastguard.tlds :refer [mutate-using-tlds]]))

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

(defn flatten-endpoint-mutations [mutations]
  (if (empty? mutations)
    [""]
    (for [head (first mutations)
          tail (flatten-endpoint-mutations (rest mutations))]
      (str head "." tail))))

(defn homoglyph-mutator [endpoint]
  (let [deconstructed-endpoint (deconstruct-endpoint endpoint)]
    (map mutate-using-homoglyphs (butlast deconstructed-endpoint))))

(defn tlds-mutator [_] 
  (mutate-using-tlds))

(defn compose-mutators [mutators endpoint]
  (apply concat (map #(% endpoint) mutators)))

(defn hostnames-exist? [endpoint-mutations]
  (pmap hostname-exists? endpoint-mutations))

(defn valid-endpoint? [checked-endpoints]
  (filter (fn [x]
            (true? (last x))) checked-endpoints))

(defn run [endpoint]
  (valid-endpoint? 
   (hostnames-exist?
    (flatten-endpoint-mutations
     (compose-mutators [homoglyph-mutator tlds-mutator] endpoint)))))