(defproject coastguard "0.1.0-SNAPSHOT"
  :description "Coastguard tries to enumerate phishing sites based on a website"
  :url "http://github.com/fmarl/coastguard"
  :license {:name "The MIT License (MIT)"
            :url "https://opensource.org/licenses/MIT"}
  :dependencies [[org.clojure/clojure "1.11.1"]
                 [org.clojure/tools.cli "1.0.206"]]
  :main ^:skip-aot coastguard.core
  :target-path "target/%s"
  :profiles {:uberjar {:aot :all
                       :jvm-opts ["-Dclojure.compiler.direct-linking=true"]}})
