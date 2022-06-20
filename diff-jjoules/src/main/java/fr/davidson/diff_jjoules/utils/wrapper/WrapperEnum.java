package fr.davidson.diff_jjoules.utils.wrapper;

import fr.davidson.diff_jjoules.utils.wrapper.maven.MavenWrapper;
import fr.davidson.diff_jjoules.utils.wrapper.properties.PropertiesWrapper;

/**
 * @author Benjamin DANGLOT
 * benjamin.danglot@davidson.fr
 * on 23/11/2021
 */
public enum WrapperEnum {

    MAVEN() {
        @Override
        public Wrapper getWrapper() {
            return new MavenWrapper();
        }
    },
    PROPERTIES() {
        @Override
        public Wrapper getWrapper() {
            return new PropertiesWrapper();
        }
    };

    public abstract Wrapper getWrapper();
}
